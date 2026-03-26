# YVR1 Edge Lag Investigation

## 背景

在 YVR 1 / Play For Dream MR 设备上，ALVR 能正常连接和显示，但用户在左右转头时会感到画面边缘明显拖拽；画面中心相对正常。这个问题在无线串流和有线串流下都存在，因此基本可以排除网络链路本身。

## 现象

- 整体画面并不是整幅一起“晚到”，主要是视野边缘出现明显滞后感。
- 编码、传输、解码延迟都不高，但主观体验很差。
- 服务端统计里曾出现异常大的“总延迟”，但进一步排查后发现这并不是唯一根因。
- 设备 OpenXR runtime 可以正常运行，APK 也能启动，但运行时行为与常见 Quest/Pico 路径不同。

## 关键排查结论

### 1. EXE / SteamVR 端基本正常

通过对 `server_openvr` 链路打日志，可以确认：

- 发送到 SteamVR 的 `local view params` 已经是正交化后的左右眼视图。
- 每帧送编码前的 `global view params` 会随着头部姿态正常变化。
- 因此问题不主要出在 Windows streamer / SteamVR 的编码前视图合成阶段。

### 2. YVR runtime 存在独立时间域

在客户端同时记录以下数据后发现：

- `xr_instance.now()` 得到的时间大约落在 `2985x s`
- `xrWaitFrame().predicted_display_time` / `predicted_display_period` 对应的时间大约落在 `5396x s`

这说明 YVR runtime 给出的“当前时间”和“显示预测时间”不在同一个时间域。

而 ALVR 原有逻辑里：

- tracking 使用 `xr_instance.now()` 推导 `target_time`
- render / `end_frame()` 使用 `predicted_display_time`

这会导致：

- 头部/手部 tracking 查询使用了一套时钟
- 最终 runtime 重投影和显示又使用另一套时钟

结果就是 runtime 在对一张“姿态与显示时间不匹配”的帧做补偿。这个问题最容易表现为画面中心相对可接受，但边缘拖拽、错位、重投影感非常明显。

## 最终修复

### 1. YVR 1 loader 选择

针对 YVR 1 机型，Android 客户端需要加载 `libopenxr_loader_yvr1.so`，否则应用会在启动阶段崩溃。

### 2. YVR 强制走客户端自定义重投影

YVR / PFDMR 这类 runtime 对 `XrCompositionLayerProjectionView` 的处理不可靠，因此客户端需要继续使用 ALVR 自己的重投影路径，而不能完全依赖 runtime。

### 3. 发送给 SteamVR 的视图先做正交化

本地 canted view 不能直接原样发给 SteamVR。发送前需要先转换成正交视图，再在客户端重投影回实际头显视图。

### 4. 为 YVR 增加时间域校正

这是最终命中根因的修复。

思路：

- 在 render 阶段，用 `predicted_display_time - xr_instance.now() - frame_interval` 估算 YVR runtime 的时间域偏移
- 将这个偏移缓存下来
- 在 tracking 阶段，不再直接使用 `xr_instance.now()`，而是先应用这个 bias，再计算 `target_time`
- 同时 `vsync_queue` 也改为基于校正后的 `now` 计算

这样可以让 tracking 查询、runtime 视图定位、frame submit 尽量落在同一时间语义下。

## 相关文件

- `alvr/client_openxr/src/lib.rs`
  - YVR 1 loader 选择
- `alvr/client_core/src/lib.rs`
  - 本地 view params 正交化后再发给服务端
- `alvr/client_openxr/src/stream.rs`
  - YVR 自定义重投影
  - YVR 时间域 bias 校正
  - 调试日志
- `alvr/server_openvr/src/lib.rs`
  - 服务端视图链路调试日志

## 调试方法总结

为了定位问题，额外加入了三类日志：

1. 服务端视图日志
   - `YVR TRACE server local views`
   - `YVR TRACE server frame views`

2. 客户端渲染日志
   - `YVR TRACE client prerender`
   - `YVR TRACE client render`

3. 客户端 tracking 日志
   - `YVR DEBUG tracking`

其中最关键的一条证据是：同一帧里 `frame_ts` / tracking `now` 与 `vsync_time` 明显处于不同时间尺度，从而暴露了 runtime 时间域不一致问题。

## 验证结果

最终用户反馈修复后“完美”，说明：

- 视野边缘拖拽问题已消失或显著恢复正常
- 主观体验已经达到可接受水平
- 该问题的主因确实是 YVR runtime 的时间域/重投影语义与通用 OpenXR 路径不完全兼容

## 后续建议

- 后续可以将 YVR 专用调试日志收敛为可配置开关，避免长期保留过多运行时输出。
- 如果未来适配更多 PFDMR/YVR 派生设备，可以考虑把这套时间域校正逻辑抽成平台特定策略，而不是散落在通用 stream 路径里。
