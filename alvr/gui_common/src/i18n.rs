use std::{
    borrow::Cow,
    sync::atomic::{AtomicU8, Ordering},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    English = 0,
    Chinese = 1,
}

impl Language {
    pub const ALL: [Self; 2] = [Self::English, Self::Chinese];

    pub fn label(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Chinese => "中文",
        }
    }

    pub fn translate<'a>(self, text: &'a str) -> Cow<'a, str> {
        if matches!(self, Self::English) {
            return Cow::Borrowed(text);
        }

        if let Some(translated) = translate_exact(text) {
            return Cow::Borrowed(translated);
        }

        let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if let Some(translated) = translate_normalized(&normalized) {
            return Cow::Borrowed(translated);
        }

        Cow::Borrowed(text)
    }
}

fn translate_exact(text: &str) -> Option<&'static str> {
    Some(match text {
        "Devices" => "设备",
        "Statistics" => "统计",
        "Settings" => "设置",
        "Installation" => "安装",
        "Logs" => "日志",
        "Debug" => "调试",
        "About" => "关于",
        "Language" => "语言",
        "Video" => "视频",
        "Audio" => "音频",
        "Headset" => "头显",
        "Connection" => "连接",
        "Extra" => "附加",
        "SteamVR is restarting" => "SteamVR 正在重启",
        "Restart SteamVR" => "重启 SteamVR",
        "Launch SteamVR" => "启动 SteamVR",
        "SteamVR:" => "SteamVR：",
        "Connected" => "已连接",
        "Disconnected" => "未连接",
        "Connecting" => "连接中",
        "Streaming" => "串流中",
        "Disconnecting" => "断开中",
        "Error" => "错误",
        "Warning" => "警告",
        "Info" => "信息",
        "Presets" => "预设",
        "Welcome to ALVR" => "欢迎使用 ALVR",
        "This setup wizard will help you setup ALVR." => {
            "这个设置向导将帮助你完成 ALVR 的初始配置。"
        }
        "Reset settings" => "重置设置",
        "It is recommended to reset your settings everytime you update ALVR." => {
            "每次更新 ALVR 后，建议重置一次设置。"
        }
        "Hardware requirements" => "硬件要求",
        "Software requirements" => "软件要求",
        "Unsupported OS" => "不支持的操作系统",
        "Download Virtual Audio Cable (Lite)" => "下载 Virtual Audio Cable（Lite）",
        "Firewall" => "防火墙",
        "Add firewall rules" => "添加防火墙规则",
        "Recommendations" => "建议",
        "Finished" => "完成",
        "Finish" => "完成",
        "Next" => "下一步",
        "Back" => "上一步",
        "Edit connection" => "编辑连接",
        "Hostname:" => "主机名：",
        "IP Addresses:" => "IP 地址：",
        "Add new" => "新增",
        "Cancel" => "取消",
        "Save" => "保存",
        "Wired Connection" => "有线连接",
        "ADB download progress" => "ADB 下载进度",
        "New Wireless Devices" => "新发现的无线设备",
        "Trust" => "信任",
        "Trusted Wireless Devices" => "已信任的无线设备",
        "Add device manually" => "手动添加设备",
        "Unknown IP" => "未知 IP",
        "Remove" => "移除",
        "Edit" => "编辑",
        "Run setup wizard" => "运行设置向导",
        "Remove firewall rules" => "移除防火墙规则",
        "Registered drivers" => "已注册驱动",
        "Register ALVR driver" => "注册 ALVR 驱动",
        "Visit us on GitHub" => "访问 GitHub",
        "Join us on Discord" => "加入 Discord",
        "Latest release" => "最新版本",
        "Donate to ALVR on Open Collective" => "在 Open Collective 支持 ALVR",
        "License:" => "许可证：",
        "Capture frame" => "截取画面",
        "Insert IDR" => "插入 IDR",
        "Start recording" => "开始录制",
        "Stop recording" => "停止录制",
        "Copy all" => "复制全部",
        "Open logs directory" => "打开日志目录",
        "Clear all" => "清空全部",
        "No new notifications" => "暂无新通知",
        "Expand" => "展开",
        "Collapse" => "折叠",
        "Reduce" => "收起",
        "Tip" => "提示",
        "New ALVR version available" => "发现新的 ALVR 版本",
        "Don't remind me again for this version" => "这个版本不再提醒",
        "You can download this version using the launcher:" => "你可以通过启动器下载这个版本：",
        "Open Launcher" => "打开启动器",
        "Download Launcher" => "下载启动器",
        "Releases page" => "版本发布页",
        "Latency" => "延迟",
        "Motion to Photon Latency" => "动作到显示延迟",
        "ALVR Latency" => "ALVR 延迟",
        "Client System (not ALVR latency)" => "客户端系统（不计入 ALVR 延迟）",
        "Client App Compositor" => "客户端合成器",
        "Frame Buffering" => "帧缓冲",
        "Decode" => "解码",
        "Network" => "网络",
        "Encode" => "编码",
        "Streamer Compositor" => "串流端合成器",
        "Game Render (not ALVR latency)" => "游戏渲染（不计入 ALVR 延迟）",
        "Framerate" => "帧率",
        "Server FPS" => "服务器 FPS",
        "Client FPS" => "客户端 FPS",
        "Bitrate and Throughput" => "码率与吞吐量",
        "Initial calculated throughput" => "初始计算吞吐量",
        "Encoder latency limiter" => "编码器延迟限制",
        "Network latency limiter" => "网络延迟限制",
        "Decoder latency limiter" => "解码器延迟限制",
        "Manual max throughput" => "手动最大吞吐量",
        "Manual min throughput" => "手动最小吞吐量",
        "Requested bitrate" => "请求码率",
        "Recorded throughput" => "记录吞吐量",
        "Recorded bitrate" => "记录码率",
        "Total packets:" => "总数据包：",
        "Total sent:" => "总发送量：",
        "Bitrate:" => "码率：",
        "Total latency:" => "总延迟：",
        "Encoder latency:" => "编码延迟：",
        "Transport latency:" => "传输延迟：",
        "Decoder latency:" => "解码延迟：",
        "Client FPS:" => "客户端 FPS：",
        "Streamer FPS:" => "串流端 FPS：",
        "Headset battery" => "头显电量",
        "plugged" => "已接电源",
        "unplugged" => "未接电源",
        "Preset not applied" => "未应用预设",
        "Unimplemented UI" => "尚未实现的界面",
        "Reset to" => "重置为",
        "ON" => "开",
        "OFF" => "关",
        "Default" => "默认",
        "Set" => "设定",
        "default list" => "默认列表",
        "Add element" => "添加元素",
        "Add entry" => "添加条目",
        "Resolution" => "分辨率",
        "Very Low (width: 3072)" => "很低（宽度：3072）",
        "Low (width: 3712)" => "低（宽度：3712）",
        "Medium (width: 4288)" => "中（宽度：4288）",
        "High (width: 5184)" => "高（宽度：5184）",
        "Ultra (width: 5632)" => "超高（宽度：5632）",
        "Extreme (width: 6080)" => "极限（宽度：6080）",
        "Preferred framerate" => "首选帧率",
        "Codec preset" => "编解码预设",
        "Encoder preset" => "编码器预设",
        "Speed" => "速度",
        "Balanced" => "均衡",
        "Quality" => "质量",
        "Foveation preset" => "注视点渲染预设",
        "Light" => "轻度",
        "Medium" => "中等",
        "High" => "高",
        "Headset speaker" => "头显扬声器",
        "Headset microphone" => "头显麦克风",
        "Disabled" => "禁用",
        "Enabled" => "启用",
        "System Default" => "系统默认",
        "Automatic" => "自动",
        "Virtual Audio Cable" => "Virtual Audio Cable",
        "VB Cable" => "VB Cable",
        "VoiceMeeter" => "VoiceMeeter",
        "VoiceMeeter Aux" => "VoiceMeeter Aux",
        "VoiceMeeter VAIO3" => "VoiceMeeter VAIO3",
        "Hand tracking interaction" => "手部追踪交互",
        "SteamVR Input 2.0" => "SteamVR 输入 2.0",
        "SteamVR input 2.0" => "SteamVR 输入 2.0",
        "ALVR bindings" => "ALVR 绑定",
        "Eye and face tracking" => "眼动与面部追踪",
        "VRChat Eye OSC" => "VRChat Eye OSC",
        "VRCFaceTracking" => "VRCFaceTracking",
        "This setting can be changed in real-time during streaming!" => {
            "该设置可以在串流过程中实时修改！"
        }
        "OK" => "确定",
        "Close" => "关闭",
        "Add version" => "添加版本",
        "Channel" => "通道",
        "Stable" => "稳定版",
        "Nightly" => "每夜版",
        "Version" => "版本",
        "Copy session from:" => "复制配置自：",
        "None" => "无",
        "Install" => "安装",
        "Edit version" => "编辑版本",
        "Delete version" => "删除版本",
        "Are you sure?" => "确定吗？",
        "ALVR Launcher" => "ALVR 启动器",
        "Fetching latest release..." => "正在获取最新版本...",
        "Open directory" => "打开目录",
        "Install APK" => "安装 APK",
        "Launch" => "启动",
        "Error!" => "错误！",
        "Failed to get release info" => "获取版本信息失败",
        "Passthrough" => "透视",
        "Blend" => "混合",
        "Premultiplied alpha" => "预乘 Alpha",
        "Threshold" => "阈值",
        "Bitrate" => "码率",
        "Mode" => "模式",
        "Adaptive" => "自适应",
        "Saturation multiplier" => "饱和度乘数",
        "Max saturation multiplier" => "最大饱和度乘数",
        "Adapt to framerate" => "适应帧率",
        "History size" => "历史长度",
        "Image corruption fix" => "图像损坏修复",
        "Preferred codec" => "首选编解码器",
        "Foveated encoding" => "注视点编码",
        "Force enable" => "强制启用",
        "Color correction" => "色彩校正",
        "Brightness" => "亮度",
        "Contrast" => "对比度",
        "Saturation" => "饱和度",
        "Gamma" => "伽马",
        "Sharpening" => "锐化",
        "Buffering history weight" => "缓冲历史权重",
        "Enforce server frame pacing" => "强制服务器帧节奏",
        "Encoder config" => "编码器配置",
        "Rate control mode" => "码率控制模式",
        "Entropy coding" => "熵编码",
        "Filler data" => "填充数据",
        "NVENC" => "NVENC",
        "P1" => "P1",
        "P2" => "P2",
        "P3" => "P3",
        "P4" => "P4",
        "P5" => "P5",
        "P6" => "P6",
        "P7" => "P7",
        "Tuning preset" => "调优预设",
        "High quality" => "高质量",
        "Low latency" => "低延迟",
        "Ultra low latency" => "超低延迟",
        "Lossless" => "无损",
        "Multi pass" => "多次编码",
        "Adaptive quantization mode" => "自适应量化模式",
        "Spatial" => "空间",
        "Temporal" => "时间",
        "Enable intra refresh" => "启用帧内刷新",
        "Intra refresh period" => "帧内刷新周期",
        "Intra refresh count" => "帧内刷新计数",
        "Max num ref frames" => "最大参考帧数",
        "Gop length" => "GOP 长度",
        "P frame strategy" => "P 帧策略",
        "Low delay key frame scale" => "低延迟关键帧缩放",
        "Refresh rate" => "刷新率",
        "Rc buffer size" => "RC 缓冲区大小",
        "Rc initial delay" => "RC 初始延迟",
        "Rc max bitrate" => "RC 最大码率",
        "Rc average bitrate" => "RC 平均码率",
        "Enable weighted prediction" => "启用加权预测",
        "AMF" => "AMF",
        "Use preproc" => "使用预处理",
        "Preproc sigma" => "预处理 sigma",
        "Preproc tor" => "预处理 tor",
        "Software (CPU) encoding" => "软件（CPU）编码",
        "Force software encoding" => "强制软件编码",
        "Force software decoder" => "强制软件解码器",
        "Mediacodec extra options" => "Mediacodec 额外选项",
        "Key" => "键",
        "Type" => "类型",
        "Float" => "浮点数",
        "Int32" => "Int32",
        "Int64" => "Int64",
        "String" => "字符串",
        "Value" => "值",
        "Red" => "红色",
        "Green" => "绿色",
        "Blue" => "蓝色",
        "Feathering" => "羽化",
        "Saturation start max" => "饱和度起始最大值",
        "Saturation start min" => "饱和度起始最小值",
        "Saturation end min" => "饱和度结束最小值",
        "Saturation end max" => "饱和度结束最大值",
        "Value start max" => "明度起始最大值",
        "Value start min" => "明度起始最小值",
        "Value end min" => "明度结束最小值",
        "Value end max" => "明度结束最大值",
        "Transcoding view resolution" => "转码视图分辨率",
        "Scale" => "缩放",
        "Absolute" => "绝对值",
        "Width" => "宽度",
        "Height" => "高度",
        "Emulated headset view resolution" => "模拟头显视图分辨率",
        "Adapter index" => "适配器索引",
        "Client-side foveation" => "客户端侧注视点渲染",
        "Static" => "静态",
        "Level" => "级别",
        "Dynamic" => "动态",
        "Max level" => "最大级别",
        "Client-side post-processing" => "客户端侧后处理",
        "Super sampling" => "超采样",
        "Normal" => "普通",
        "Upscaling" => "超分辨率提升",
        "Edge direction" => "边缘方向",
        "Edge threshold" => "边缘阈值",
        "Edge sharpness" => "边缘锐度",
        "Upscale factor" => "放大系数",
        "Device" => "设备",
        "Buffering" => "缓冲",
        "1/4 resolution" => "1/4 分辨率",
        "Enable High-Motion Quality Boost" => "启用高速运动画质增强",
        "Enable Pre-analysis" => "启用预分析",
        "Encoder thread count" => "编码线程数",
        "Enable HDR" => "启用 HDR",
        "Force HDR sRGB Correction" => "强制 HDR sRGB 校正",
        "Clamp HDR extended range" => "限制 HDR 扩展范围",
        "Quality preset" => "质量预设",
        "Enable VBAQ/CAQ" => "启用 VBAQ/CAQ",
        "h264: Profile" => "h264：配置文件",
        "10-bit encoding" => "10 位编码",
        "Encoding Gamma" => "编码伽马",
        "Constant" => "固定",
        "FPS reset threshold multiplier" => "FPS 重置阈值倍率",
        "Custom" => "自定义",
        "Sink" => "输出端",
        "Source" => "输入源",
        "By name (substring)" => "按名称（子串）",
        "By index" => "按索引",
        "Average buffering" => "平均缓冲",
        "Batch size" => "批大小",
        "Mute desktop audio when streaming" => "串流时静音桌面音频",
        "Rift S" => "Rift S",
        "Quest 1" => "Quest 1",
        "Quest 2" => "Quest 2",
        "Quest Pro" => "Quest Pro",
        "Pico 4" => "Pico 4",
        "Power Saving" => "省电",
        "Sustained Low" => "持续低性能",
        "Sustained High" => "持续高性能",
        "CPU" => "CPU",
        "GPU" => "GPU",
        "Body Tracking" => "身体追踪",
        "Object Tracking" => "物体追踪",
        "Fake Vive Trackers" => "虚拟 Vive 追踪器",
        "VRChat Body OSC" => "VRChat Body OSC",
        "Rift S Touch" => "Rift S Touch",
        "Quest 1 Touch" => "Quest 1 Touch",
        "Quest 2 Touch" => "Quest 2 Touch",
        "Quest 3 Touch Plus" => "Quest 3 Touch Plus",
        "PSVR2 Sense Controller" => "PSVR2 Sense Controller",
        "Valve Index" => "Valve Index",
        "Vive Wand" => "Vive Wand",
        "Vive Tracker" => "Vive Tracker",
        "Minimum duration" => "最短持续时间",
        "Prediction" => "预测",
        "Extra OpenVR properties" => "额外 OpenVR 属性",
        "Map non-held controllers to SteamVR trackers" => "将未持握控制器映射为 SteamVR 追踪器",
        "UDP" => "UDP",
        "TCP" => "TCP",
        "Send size" => "发送缓冲大小",
        "Receive size" => "接收缓冲大小",
        "Allow untrusted HTTP" => "允许不受信任的 HTTP",
        "Local OSC port" => "本地 OSC 端口",
        "Minimum IDR interval" => "最小 IDR 间隔",
        "DSCP (packet prio hints)" => "DSCP（数据包优先级提示）",
        "Open and close SteamVR automatically" => "自动打开和关闭 SteamVR",
        "Duration" => "时长",
        "Start video recording at client connection" => "客户端连接时开始录制视频",
        "Maximum bitrate" => "最大码率",
        "Minimum bitrate" => "最小码率",
        "Maximum network latency" => "最大网络延迟",
        "Maximum decoder latency" => "最大解码延迟",
        "latency overstep" => "延迟超限",
        "Foveation offset" => "注视点偏移",
        "Center region width" => "中心区域宽度",
        "Center region height" => "中心区域高度",
        "Center shift X" => "中心偏移 X",
        "Center shift Y" => "中心偏移 Y",
        "Horizontal edge ratio" => "水平边缘比例",
        "Vertical edge ratio" => "垂直边缘比例",
        "h264" => "h264",
        "HEVC" => "HEVC",
        "AV1" => "AV1",
        "Main" => "Main",
        "Baseline" => "Baseline",
        "Hue start max" => "色相起始最大值",
        "Hue start min" => "色相起始最小值",
        "Hue end min" => "色相结束最小值",
        "Hue end max" => "色相结束最大值",
        "RGB Chroma Key" => "RGB 色度键",
        "HSV Chroma Key" => "HSV 色度键",
        "Maximum buffering" => "最大缓冲",
        "Preferred FPS" => "首选 FPS",
        "Position recentering mode" => "位置重定位模式",
        "Rotation recentering mode" => "旋转重定位模式",
        "Local floor" => "本地地面",
        "Local" => "本地",
        "View height" => "视图高度",
        "Yaw" => "偏航",
        "Tilted" => "倾斜",
        "Controllers" => "控制器",
        "Tracked" => "启用追踪",
        "Hand skeleton" => "手部骨骼",
        "Predict" => "预测",
        "Only touch" => "仅触摸",
        "Pinch touch distance" => "捏合触摸距离",
        "Pinch trigger distance" => "捏合扳机距离",
        "Curl touch distance" => "弯曲触摸距离",
        "Curl trigger distance" => "弯曲扳机距离",
        "Joystick deadzone" => "摇杆死区",
        "Joystick offset horizontal" => "摇杆水平偏移",
        "Joystick offset vertical" => "摇杆垂直偏移",
        "Joystick range" => "摇杆范围",
        "Activation delay" => "激活延迟",
        "Deactivation delay" => "停用延迟",
        "Repeat delay" => "重复延迟",
        "Haptics" => "触觉反馈",
        "Intensity multiplier" => "强度乘数",
        "Amplitude curve" => "振幅曲线",
        "Emulation mode" => "模拟模式",
        "Serial number" => "序列号",
        "Button set" => "按键集合",
        "Linear velocity cutoff" => "线速度截止值",
        "Angular velocity cutoff" => "角速度截止值",
        "Left controller position offset" => "左控制器位置偏移",
        "Left controller rotation offset" => "左控制器旋转偏移",
        "Left hand tracking position offset" => "左手追踪位置偏移",
        "Left hand tracking rotation offset" => "左手追踪旋转偏移",
        "Button mappings" => "按键映射",
        "Button mapping config" => "按键映射配置",
        "Destination" => "目标",
        "Mapping type" => "映射类型",
        "Binary conditions" => "二进制条件",
        "Hysteresis threshold" => "滞回阈值",
        "Binary to scalar" => "二进制转标量",
        "Remap" => "重映射",
        "Click threshold" => "点击阈值",
        "Touch threshold" => "触摸阈值",
        "Force threshold" => "力度阈值",
        "Deviation" => "偏差",
        "Vive" => "Vive",
        "Performance level" => "性能等级",
        "Tracking ref only" => "仅作为追踪参考",
        "Enable vive tracker proxy" => "启用 Vive Tracker 代理",
        "Face tracking" => "面部追踪",
        "Sources" => "来源",
        "Prefer eye tracking only" => "优先仅眼动追踪",
        "Prefer full face tracking" => "优先完整面部追踪",
        "Body tracking" => "身体追踪",
        "Meta" => "Meta",
        "Prefer full body" => "优先完整身体追踪",
        "Prefer high fidelity" => "优先高保真",
        "Bd" => "BD",
        "High accuracy" => "高精度",
        "Prompt calibration on start" => "启动时提示校准",
        "Multimodal tracking" => "多模态追踪",
        "VMC" => "VMC",
        "Host" => "主机",
        "Port" => "端口",
        "Publish" => "发布",
        "Orientation correction" => "方向校正",
        "Max prediction ms" => "最大预测时间（毫秒）",
        "Stream protocol" => "串流协议",
        "Client discovery" => "客户端发现",
        "Auto trust clients" => "自动信任客户端",
        "Wired client type" => "有线客户端类型",
        "Wired client autolaunch" => "有线客户端自动启动",
        "Boot delay" => "启动延迟",
        "Enable on connect script" => "启用连接时脚本",
        "Enable on disconnect script" => "启用断开时脚本",
        "Avoid video glitching" => "避免视频故障",
        "Packet size" => "数据包大小",
        "Stream port" => "串流端口",
        "Web server port" => "Web 服务器端口",
        "Server buffer config" => "服务器缓冲配置",
        "Client buffer config" => "客户端缓冲配置",
        "Maximum" => "最大值",
        "Max queued server video frames" => "服务器视频最大排队帧数",
        "Statistics history size" => "统计历史长度",
        "Best effort" => "尽力而为",
        "Class selector" => "类别选择器",
        "Assured forwarding" => "保证转发",
        "Class" => "类别",
        "Drop probability" => "丢包概率",
        "Expedited forwarding" => "加速转发",
        "Logging" => "日志",
        "Show notification tip" => "显示通知提示",
        "Prefer backtrace" => "优先回溯信息",
        "Notification level" => "通知级别",
        "Client log report level" => "客户端日志上报级别",
        "Show raw events" => "显示原始事件",
        "Hide spammy events" => "隐藏高频事件",
        "Log to disk" => "写入磁盘日志",
        "Log tracking" => "记录追踪日志",
        "Log button presses" => "记录按键日志",
        "Log haptics" => "记录触觉反馈日志",
        "Patches" => "补丁",
        "Linux async compute" => "Linux 异步计算",
        "Linux async reprojection" => "Linux 异步重投影",
        "Direct launch" => "直接启动",
        "Capture" => "录制",
        "Rolling video files" => "滚动视频文件",
        "Capture frame dir" => "截帧目录",
        "SteamVR Launcher" => "SteamVR 启动器",
        "New version popup" => "新版本弹窗",
        "Hide while version" => "忽略该版本",
        "Store" => "商店版",
        "Github" => "GitHub 版",
        _ => return None,
    })
}

fn translate_normalized(text: &str) -> Option<&'static str> {
    Some(match text {
        "ALVR requires a dedicated and recent graphics card. Low-end Intel integrated graphics may fail to work. Make sure you have at least one output audio device." => {
            "ALVR 需要较新的独立显卡。低端 Intel 核显可能无法正常工作。请确认系统至少有一个音频输出设备。"
        }
        "To stream the headset microphone on Windows you need to install Virtual Audio Cable, VB-Cable, Voicemeeter" => {
            "在 Windows 上串流头显麦克风时，你需要安装 Virtual Audio Cable、VB-Cable 或 Voicemeeter。"
        }
        "You need the PipeWire (0.3.49+ version) audio system to be able to stream audio and use microphone." => {
            "你需要安装 PipeWire（0.3.49 及以上版本）音频系统，才能串流音频并使用麦克风。"
        }
        "To communicate with the headset, some firewall rules need to be set. This requires administrator rights!" => {
            "为了与头显通信，需要配置一些防火墙规则。这需要管理员权限！"
        }
        "ALVR supports multiple types of PC hardware and headsets but not all might work correctly with default settings. Please try tweaking different settings like resolution, bitrate, encoder and others if your ALVR experience is not optimal." => {
            "ALVR 支持多种 PC 硬件和头显，但默认设置不一定适合所有设备。如果你的 ALVR 体验不理想，请尝试调整分辨率、码率、编码器等设置。"
        }
        "You can always restart this setup wizard from the \"Installation\" tab on the left." => {
            "你随时都可以在左侧的“安装”标签页中重新打开这个设置向导。"
        }
        "ALVR requires running SteamVR! Devices will not be discovered or connected." => {
            "ALVR 需要先运行 SteamVR！否则无法发现或连接设备。"
        }
        "Recording from ALVR using the buttons below is not suitable for capturing gameplay. For that, use other means of recording, for example through headset or desktop VR output." => {
            "使用下面这些按钮从 ALVR 录制的内容并不适合直接录制游戏画面。如果需要录制游戏，请使用其他录制方式，例如头显端或桌面 VR 输出。"
        }
        "No statistics available. Start SteamVR and connect to a device to gather statistics." => {
            "暂无统计数据。请先启动 SteamVR 并连接设备以采集统计信息。"
        }
        "Note: throughput is the peak bitrate, packet_size/network_latency." => {
            "注意：吞吐量表示峰值码率，即 packet_size/network_latency。"
        }
        "Choosing too high resolution (commonly 'High (width: 5184)') may result in high latency or black screen." => {
            "分辨率设置过高（通常如“高（宽度：5184）”）可能导致延迟升高或黑屏。"
        }
        "Selecting a quality too high may result in stuttering or still image!" => {
            "选择过高的画质可能导致卡顿或画面静止！"
        }
        "You can change the default audio device from the system taskbar tray (bottom right)" => {
            "你可以在系统任务栏托盘（右下角）中更改默认音频设备。"
        }
        "Changing this setting will make SteamVR restart! Please save your in-game progress first" => {
            "修改该设置将导致 SteamVR 重启！请先保存你的游戏进度。"
        }
        "Foveation affects pixelation on the edges of the screen and significantly reduces codec latency. It is not recommended to fully disable it, as it may cause shutterring and high encode/decode latency!" => {
            "注视点渲染会影响画面边缘的像素化效果，并显著降低编解码延迟。不建议完全关闭，否则可能导致抖动以及较高的编解码延迟！"
        }
        "AV1 encoding is only supported on RDNA3, Ada Lovelace, Intel ARC or newer GPUs (AMD RX 7xxx+ , NVIDIA RTX 40xx+, Intel ARC) and on headsets that have XR2 Gen 2 onboard (Quest 3, Pico 4 Ultra). H264 encoding is currently NOT supported on Intel ARC GPUs on Windows." => {
            "AV1 编码仅支持 RDNA3、Ada Lovelace、Intel ARC 或更新的显卡（AMD RX 7xxx+、NVIDIA RTX 40xx+、Intel ARC），以及搭载 XR2 Gen 2 的头显（Quest 3、Pico 4 Ultra）。当前在 Windows 上，Intel ARC GPU 仍不支持 H264 编码。"
        }
        "Disabled: hands cannot emulate buttons. Useful for using Joy-Cons or other non-native controllers. SteamVR Input 2.0: create separate SteamVR devices for hand tracking. ALVR bindings: use ALVR hand tracking button bindings. Check the wiki for help." => {
            "禁用：手部无法模拟按键，适合配合 Joy-Con 或其他非原生控制器使用。SteamVR Input 2.0：为手部追踪创建独立的 SteamVR 设备。ALVR 绑定：使用 ALVR 的手部追踪按键绑定，详情请查看 Wiki。"
        }
        "If you started having crashes after changing some settings, reset ALVR by re-running \"Run setup wizard\" from the \"Installation\" tab and clicking \"Reset settings\"." => {
            "如果你在修改某些设置后开始出现崩溃，请到“安装”标签页重新运行“设置向导”，然后点击“重置设置”来恢复 ALVR。"
        }
        "Some settings are hidden by default. Click the \"Expand\" button next to some settings to expand the submenus." => {
            "有些设置默认会隐藏。点击设置旁边的“展开”按钮即可展开子菜单。"
        }
        "It's highly advisable to keep audio settings as default in ALVR and modify the default audio device in the taskbar tray." => {
            "强烈建议在 ALVR 中保持默认音频设置，并在任务栏托盘中修改系统默认音频设备。"
        }
        "Increasing \"Video\"->\"Maximum buffering\" may reduce stutters at the cost of more latency." => {
            "增大“Video”->“Maximum buffering”可以减少卡顿，但会带来更高延迟。"
        }
        "Sometimes switching between h264 and HEVC codecs is necessary on certain GPUs to fix crashing or fallback to software encoding." => {
            "在某些显卡上，切换 h264 与 HEVC 编解码器有时可以解决崩溃或回退到软件编码的问题。"
        }
        "If you're using an NVIDIA GPU, it's best to use high-bitrate H264; if you're using an AMD GPU, HEVC might look better." => {
            "如果你使用 NVIDIA GPU，通常高码率 H264 效果更好；如果你使用 AMD GPU，HEVC 可能观感更佳。"
        }
        "If you experience \"white snow\" flickering, set \"Presets\"->\"Resolution\" to \"Low\" and disable \"Video\"->\"Foveated encoding\"." => {
            "如果你遇到“白雪花”闪烁问题，请将“Presets”->“Resolution”设为“Low”，并关闭“Video”->“Foveated encoding”。"
        }
        "Increasing \"Video\"->\"Color correction\"->\"Sharpness\" may improve the perceived image quality." => {
            "增大“Video”->“Color correction”->“Sharpness”可能会提升主观画质。"
        }
        "If you have problems syncing external controllers or trackers to ALVR tracking space, add one element to \"Headset\"->\"Extra OpenVR properties\", then set a custom \"Tracking system name string\"." => {
            "如果你在将外部控制器或追踪器同步到 ALVR 追踪空间时遇到问题，可以在“Headset”->“Extra OpenVR properties”中添加一个条目，然后设置自定义“Tracking system name string”。"
        }
        "To change the visual appearance of controllers, set \"Headset\"->\"Controllers\"->\"Emulation mode\"." => {
            "若要更改控制器的外观，请设置“Headset”->“Controllers”->“Emulation mode”。"
        }
        "ALVR supports custom button bindings! If you need help, please ask us on our Discord server." => {
            "ALVR 支持自定义按键绑定！如果需要帮助，欢迎到我们的 Discord 服务器提问。"
        }
        "ALVR supports hand tracking gestures (\"Presets\"->\"Hand tracking interaction\"->\"ALVR bindings\"). Check out wiki how to use them properly: https://github.com/alvr-org/ALVR/wiki/Hand-tracking-controller-bindings." => {
            "ALVR 支持手部追踪手势（“Presets”->“Hand tracking interaction”->“ALVR bindings”）。请查看 Wiki 了解正确用法：https://github.com/alvr-org/ALVR/wiki/Hand-tracking-controller-bindings"
        }
        "If hand tracking gestures are annoying, you can disable them in \"Headset\"->\"Controllers\"->\"Hand tracking interaction\". Alternatively, you can enable \"Hand tracking interaction\"->\"Only touch\"." => {
            "如果你觉得手部追踪手势影响体验，可以在“Headset”->“Controllers”->“Hand tracking interaction”中禁用它们。或者你也可以启用“Hand tracking interaction”->“Only touch”。"
        }
        "You can fine-tune the controllers' responsiveness with \"Headset\"->\"Controllers\"->\"Prediction\"." => {
            "你可以通过“Headset”->“Controllers”->“Prediction”微调控制器响应速度。"
        }
        "If the visual controller/hand models do not match the physical controller's position, you can tweak the offset in \"Headset\"->\"Controllers\"->\"Left controller position/rotation offset\" (affects both controllers)." => {
            "如果虚拟控制器/手部模型与实际控制器位置不一致，可以调整“Headset”->“Controllers”->“Left controller position/rotation offset”（会同时影响左右控制器）。"
        }
        "When using external trackers or controllers, you should set both \"Headset\"->\"Position/Rotation recentering mode\" to \"Disabled\"." => {
            "使用外部追踪器或控制器时，建议将“Headset”->“Position/Rotation recentering mode”都设为“Disabled”。"
        }
        "You can enable tilt mode. Set \"Headset\"->\"Position recentering mode\" to \"Local\" and \"Headset\"->\"Rotation recentering mode\" to \"Tilted\"." => {
            "你可以启用倾斜模式。将“Headset”->“Position recentering mode”设为“Local”，并将“Headset”->“Rotation recentering mode”设为“Tilted”。"
        }
        "If you often experience image glitching, you can trade that with stutter frames using \"Connection\"->\"Avoid video glitching\"." => {
            "如果你经常遇到画面异常，可以通过“Connection”->“Avoid video glitching”用额外卡顿帧来换取更稳定的画面。"
        }
        "You can run custom commands/programs at headset connection/disconnection using \"Connection\"->\"Enable on connect/disconnect script\"." => {
            "你可以通过“Connection”->“Enable on connect/disconnect script”在头显连接/断开时运行自定义命令或程序。"
        }
        "In case you want to report a bug, to get a log file, enable \"Extra\"->\"Logging\"->\"Log to disk\". The log will be inside \"session_log.txt\"." => {
            "如果你想报告 bug，请启用“Extra”->“Logging”->“Log to disk”来生成日志文件。日志会保存在“session_log.txt”中。"
        }
        "For hacking purposes, you can enable \"Extra\"->\"Logging\"->\"Log tracking\", \"Log button presses\" and \"Log haptics\". You can get the data using a websocket at ws://localhost:8082/api/events." => {
            "如果你在做调试或开发，可以启用“Extra”->“Logging”中的“Log tracking”、“Log button presses”和“Log haptics”。然后你可以通过 ws://localhost:8082/api/events 这个 websocket 获取数据。"
        }
        "In case you want to report a bug and share your log, you should enable \"Extra\"->\"Logging\"->\"Prefer backtrace\"." => {
            "如果你想报告 bug 并分享日志，建议启用“Extra”->“Logging”->“Prefer backtrace”。"
        }
        "You can quickly cycle through tips like this one by toggling \"Extra\"->\"Logging\"->\"Show notification tip\"." => {
            "你可以通过切换“Extra”->“Logging”->“Show notification tip”快速轮换显示这类提示。"
        }
        "It's handy to enable \"Extra\"->\"SteamVR Launcher\"->\"Open and close SteamVR automatically\"." => {
            "启用“Extra”->“SteamVR Launcher”->“Open and close SteamVR automatically”通常会更方便。"
        }
        "If you want to share a video recording for reporting a bug, you can enable \"Extra\"->\"Capture\"->\"Rolling video files\" to limit the file size of the upload." => {
            "如果你想分享录屏来报告 bug，可以启用“Extra”->“Capture”->“Rolling video files”来限制上传文件大小。"
        }
        "If your headset does not appear in the device list, it might be in a different subnet. Try \"Add device manually\" with IP shown from inside device." => {
            "如果你的头显没有出现在设备列表中，它可能位于不同子网。请尝试使用设备内部显示的 IP 通过“Add device manually”手动添加。"
        }
        "P1 is the fastest preset and P7 is the preset that produces better quality. P6 and P7 are too slow to be usable." => {
            "P1 是最快的预设，P7 的画质最好。P6 和 P7 通常过慢，不适合实际使用。"
        }
        "Reduce compression artifacts at the cost of small performance penalty" => {
            "以少量性能开销减少压缩伪影。"
        }
        "Spatial: Helps reduce color banding, but high-complexity scenes might look worse. Temporal: Helps improve overall encoding quality, very small trade-off in speed." => {
            "Spatial：有助于减少色带，但在高复杂度场景下画面可能更差。Temporal：有助于提高整体编码质量，速度损失很小。"
        }
        "Enables high motion quality boost mode. Allows the encoder to perform pre-analysis the motion of the video and use the information for better encoding" => {
            "启用高速运动画质增强模式。允许编码器预分析视频运动，并利用这些信息获得更好的编码效果。"
        }
        "Enables pre-analysis during encoding. This will likely result in reduced performance, but may increase quality. Does not work with the \"Reduce color banding\" option, requires enabling \"Use preproc\"" => {
            "在编码期间启用预分析。这样通常会降低性能，但可能提升画质。该功能与“减少色带”选项不兼容，并且需要启用“使用预处理”。"
        }
        "Forces the encoder to use CPU instead of GPU" => "强制编码器使用 CPU 而不是 GPU。",
        "If the client has no preference, enables compositing VR layers to an RGBA float16 framebuffer, and doing sRGB/YUV conversions in shader code." => {
            "当客户端没有偏好时，启用将 VR 图层合成到 RGBA float16 帧缓冲区，并在着色器中执行 sRGB/YUV 转换。"
        }
        "Forces sRGB correction on all composited SteamVR layers. Useful if an HDR-injected game is too dark." => {
            "对所有合成后的 SteamVR 图层强制执行 sRGB 校正。如果 HDR 注入后的游戏过暗，这项设置会很有用。"
        }
        "Clamps HDR extended range to 0.0~1.0, useful if you only want HDR to reduce banding." => {
            "将 HDR 扩展范围限制在 0.0~1.0。如果你只是想用 HDR 来减少色带，这项设置会很有用。"
        }
        "Controls overall quality preset of the encoder. Works only on Windows AMD AMF, Linux VAAPI (AMD/Intel)." => {
            "控制编码器的整体质量预设。仅适用于 Windows 上的 AMD AMF，以及 Linux 上的 VAAPI（AMD/Intel）。"
        }
        "Enables Variance Based Adaptive Quantization on h264 and HEVC, and Content Adaptive Quantization on AV1" => {
            "在 h264 和 HEVC 上启用基于方差的自适应量化，在 AV1 上启用内容自适应量化。"
        }
        "CBR: Constant BitRate mode. This is recommended. VBR: Variable BitRate mode. Not commended because it may throw off the adaptive bitrate algorithm. This is only supported on Windows and only with AMD/Nvidia GPUs" => {
            "CBR：固定码率模式，推荐使用。VBR：可变码率模式，不推荐，因为它可能干扰自适应码率算法。该模式仅在 Windows 上、且仅支持 AMD/NVIDIA GPU。"
        }
        "Whenever possible, attempts to use this profile. May increase compatibility with varying mobile devices. Only has an effect for h264. Doesn't affect NVENC on Windows." => {
            "在可能的情况下尝试使用该配置文件。可能会提升与不同移动设备的兼容性。仅对 h264 生效，对 Windows 上的 NVENC 无效。"
        }
        "CAVLC algorithm is recommended. CABAC produces better compression but it's significantly slower and may lead to runaway latency" => {
            "推荐使用 CAVLC 算法。CABAC 压缩效果更好，但明显更慢，并可能导致延迟持续升高。"
        }
        "In CBR mode, this makes sure the bitrate does not fall below the assigned value. This is mostly useful for debugging." => {
            "在 CBR 模式下，确保码率不会低于指定值。这项设置主要用于调试。"
        }
        "Sets the encoder to use 10 bits per channel instead of 8, if the client has no preference. Does not work on Linux with Nvidia" => {
            "当客户端没有偏好时，让编码器使用每通道 10 位而不是 8 位。在 Linux + NVIDIA 上不可用。"
        }
        "To prioritize darker pixels at the expense of potentially additional banding in midtones, set to 2.2. To allow the encoder to decide priority on its own, set to 1.0." => {
            "如果想优先照顾较暗像素、并接受中间色调可能出现更多色带，可设为 2.2。如果希望编码器自行决定优先级，请设为 1.0。"
        }
        "Allowed percentage of frame interval to allocate for video encoding" => {
            "允许分配给视频编码的帧间隔百分比。"
        }
        "When the decoder latency goes above this threshold, the bitrate will be reduced" => {
            "当解码延迟超过该阈值时，码率将被降低。"
        }
        "Number of consecutive frames above the threshold to trigger a bitrate reduction" => {
            "连续多少帧超过阈值后触发码率下降。"
        }
        "Controls how much the bitrate is reduced when the decoder latency goes above the threshold" => {
            "控制当解码延迟超过阈值时，码率会降低多少。"
        }
        "Percentage of network bandwidth to allocate for video transmission" => {
            "分配给视频传输的网络带宽百分比。"
        }
        "Currently there is a bug where the decoder latency keeps rising when above a certain bitrate" => {
            "目前存在一个问题：当码率超过某个值后，解码延迟会持续升高。"
        }
        "If the framerate changes more than this factor, trigger a parameters update" => {
            "如果帧率变化超过这个倍率，就触发参数更新。"
        }
        "Ensure that the specified bitrate value is respected regardless of the framerate" => {
            "无论帧率如何变化，都尽量保证指定的码率值被遵守。"
        }
        "Controls the smoothness during calculations" => "控制计算过程中的平滑程度。",
        "When this is enabled, an IDR frame is requested after the bitrate is changed. This has an effect only on AMD GPUs." => {
            "启用后，在码率变化后会请求一个 IDR 帧。该选项仅对 AMD GPU 生效。"
        }
        "Force enable on smartphone clients" => "在手机类客户端上强制启用。",
        "The threshold is applied per-channel" => "该阈值会按每个颜色通道分别应用。",
        "Enabling this will adapt transparency based on the brightness of each pixel. This is a similar effect to AR glasses." => {
            "启用后，会根据每个像素的亮度自适应调整透明度。这种效果类似 AR 眼镜。"
        }
        "Reduce flicker for high contrast edges. Useful when the input resolution is high compared to the headset display" => {
            "减少高对比边缘的闪烁。当输入分辨率相对头显显示分辨率较高时，这项设置会很有用。"
        }
        "Improve clarity of high contrast edges and counteract blur. Useful when the input resolution is low compared to the headset display" => {
            "提高高对比边缘的清晰度并抵消模糊。当输入分辨率相对头显显示分辨率较低时，这项设置会很有用。"
        }
        "Improves visual quality by using the edge direction to upscale at a slight performance loss" => {
            "通过利用边缘方向进行放大来提升画质，同时会带来少量性能损耗。"
        }
        "Dimensional resolution multiplier, high values will cause performance issues with weaker headset hardware or higher resolutions" => {
            "分辨率维度乘数。数值过高会在较弱的头显硬件或更高分辨率下导致性能问题。"
        }
        "HEVC may provide better visual fidelity at the cost of increased encoder latency" => {
            "HEVC 可能提供更好的画质，但代价是更高的编码延迟。"
        }
        "Disabling foveated encoding may result in significantly higher encode/decode times and stuttering, or even crashing. If you want to reduce the amount of pixelation on the edges, increase the center region width and height" => {
            "禁用注视点编码可能导致编码/解码时间显著增加、出现卡顿，甚至崩溃。如果你想减少画面边缘的像素化程度，请增大中心区域的宽度和高度。"
        }
        "Increasing this value will help reduce stutter but it will increase latency" => {
            "提高这个值有助于减少卡顿，但会增加延迟。"
        }
        "This works only on Windows. It shouldn't be disabled except in certain circumstances when you know the VR game will not meet the target framerate." => {
            "该选项仅在 Windows 上有效。除非你明确知道 VR 游戏无法达到目标帧率，否则通常不应关闭。"
        }
        "Attempts to use a software decoder on the device. Slow, but may work around broken codecs." => {
            "尝试在设备端使用软件解码器。速度较慢，但可能绕过损坏或异常的编解码器实现。"
        }
        "Resolution used for encoding and decoding. Relative to a single eye view." => {
            "用于编码和解码的分辨率。相对于单眼视图。"
        }
        "This is the resolution that SteamVR will use as default for the game rendering. Relative to a single eye view." => {
            "这是 SteamVR 默认用于游戏渲染的分辨率。相对于单眼视图。"
        }
        "You probably don't want to change this. Allows for changing adapter for ALVR compositor." => {
            "你大概率不需要修改这个选项。它允许为 ALVR 合成器切换显卡适配器。"
        }
        "Hardware optimized algorithms, available on Quest and Pico headsets" => {
            "硬件优化算法，仅在 Quest 和 Pico 头显上可用。"
        }
        "Snapdragon Game Super Resolution client-side upscaling" => {
            "Snapdragon Game Super Resolution 客户端侧超分辨率。"
        }
        "This device is used by ALVR to output microphone audio" => {
            "这个设备由 ALVR 用来输出麦克风音频。"
        }
        "This device is set in SteamVR as the default microphone" => {
            "这个设备会在 SteamVR 中被设为默认麦克风。"
        }
        "To be able to use the microphone on Windows, you need to install Virtual Audio Cable" => {
            "要在 Windows 上使用麦克风，你需要安装 Virtual Audio Cable。"
        }
        "When disabling this, the client needs to be restarted for the change to be applied." => {
            "关闭该选项后，需要重启客户端才能使改动生效。"
        }
        "Prefer active upper body tracking, Quest 3 only" => {
            "优先启用上半身主动追踪，仅适用于 Quest 3。"
        }
        "Improves accuracy of the tracking at the cost of higher latency." => {
            "以更高延迟为代价，提高追踪精度。"
        }
        "If trackers have not been calibrated before, the calibration process will start after you connect to the streamer." => {
            "如果追踪器之前没有校准过，那么在连接到串流端后会开始校准流程。"
        }
        "Turn this off to temporarily pause tracking." => "关闭此项可临时暂停追踪。",
        "Turn this off to temporarily pause sending data." => "关闭此项可临时暂停发送数据。",
        "How close the tips of your fingers need to be to register a pinch click." => {
            "手指指尖需要靠得多近，才会被识别为捏合点击。"
        }
        "How close together the tips of your fingers need to be to start registering a pinch trigger pull." => {
            "手指指尖需要靠得多近，才会开始被识别为捏合扳机动作。"
        }
        "How close to your palm the tips of your fingers need to be to register a curl click." => {
            "手指指尖需要离手掌多近，才会被识别为弯曲点击。"
        }
        "How close to your palm the tips of your fingers need to be to start registering a trigger pull." => {
            "手指指尖需要离手掌多近，才会开始被识别为扳机动作。"
        }
        "The radius of motion of the joystick. The joystick can be controlled if the thumb is within 2x this range." => {
            "摇杆的动作半径。当拇指位于该范围 2 倍之内时，就可以控制摇杆。"
        }
        "How long the gesture must be continuously held before it is activated." => {
            "手势需要持续保持多长时间后才会被激活。"
        }
        "How long the gesture must be continuously released before it is deactivated." => {
            "手势需要持续释放多长时间后才会被停用。"
        }
        "How long the after the gesture has been deactivated before it can be activated again." => {
            "手势被停用后，需要等待多长时间才能再次激活。"
        }
        "Predict hand skeleton to make it less floaty. It may make hands too jittery." => {
            "预测手部骨骼以减少漂浮感，但也可能让手部动作更抖。"
        }
        "Enabling this will use separate tracker objects with the full skeletal tracking level when hand tracking is detected. This is required for VRChat hand tracking." => {
            "启用后，当检测到手部追踪时，会使用具备完整骨骼追踪级别的独立追踪器对象。这是 VRChat 手部追踪所必需的。"
        }
        "Turning this off will make the controllers appear powered off." => {
            "关闭此项会让控制器看起来像是已断电。"
        }
        "Enabling this passes skeletal hand data (finger tracking) to SteamVR." => {
            "启用后会将手部骨骼数据（手指追踪）传递给 SteamVR。"
        }
        "Enabling this allows using hand gestures to emulate controller inputs." => {
            "启用后允许使用手势来模拟控制器输入。"
        }
        "Higher values make the controllers track smoother. Technically, this is the time (counted in frames) between pose submitted to SteamVR and the corresponding virtual vsync happens. Currently this cannot be reliably estimated automatically. The correct value should be 2 but 3 is default for smoother tracking at the cost of slight lag." => {
            "更高的数值会让控制器追踪更平滑。从技术上讲，这表示将姿态提交给 SteamVR 到对应虚拟垂直同步发生之间的时间（以帧为单位）。目前这个值还不能被可靠地自动估算。理论正确值应为 2，但默认使用 3，以少量额外延迟换取更平滑的追踪。"
        }
        "Right controller offset is mirrored horizontally" => {
            "右手控制器偏移会在水平方向上镜像左手的设置。"
        }
        "List of OpenXR-syle paths" => "OpenXR 风格路径列表。",
        "Disabled: the playspace origin is determined by the room-scale guardian setup. Local floor: the origin is on the floor and resets when long pressing the oculus button. Local: the origin resets when long pressing the oculus button, and is calculated as an offset from the current head position." => {
            "Disabled：游戏空间原点由房间级 Guardian 设置决定。Local floor：原点位于地面，长按 Oculus 按钮时重置。Local：长按 Oculus 按钮时重置原点，并根据当前头部位置计算偏移。"
        }
        "Disabled: the playspace orientation is determined by the room-scale guardian setup. Yaw: the forward direction is reset when long pressing the oculus button. Tilted: the world gets tilted when long pressing the oculus button. This is useful for using VR while laying down." => {
            "Disabled：游戏空间朝向由房间级 Guardian 设置决定。Yaw：长按 Oculus 按钮时重置朝前方向。Tilted：长按 Oculus 按钮时，整个世界会被倾斜。这在躺着使用 VR 时会很有帮助。"
        }
        "Power Savings might increase latency or reduce framerate consistency but decreases temperatures and improves battery life. Sustained Low provides consistent framerates but might increase latency if necessary. Sustained High provides consistent framerates but increases temperature. This is mainly for Quest headsets, mileage may vary on other devices." => {
            "Power Savings 可能增加延迟或降低帧率稳定性，但会降低温度并改善续航。Sustained Low 会尽量维持稳定帧率，但必要时可能增加延迟。Sustained High 会维持稳定帧率，但温度会更高。该选项主要面向 Quest 头显，其他设备效果可能不同。"
        }
        "Track hand skeleton while holding controllers. This will reduce hand tracking frequency to 30Hz. Because of runtime limitations, this option is ignored when body tracking is active." => {
            "在手持控制器时继续追踪手部骨骼。这会将手部追踪频率降到 30Hz。由于运行时限制，当身体追踪启用时该选项会被忽略。"
        }
        "Maximum prediction for head and controllers. Used to avoid too much jitter during loading." => {
            "头显和控制器的最大预测时间。用于避免加载过程中出现过多抖动。"
        }
        "Allow untrusted clients to connect without confirmation. This is not recommended for security reasons." => {
            "允许不受信任的客户端在无需确认的情况下连接。出于安全原因，不推荐这样做。"
        }
        "Delay in seconds to wait after booting the headset before trying to launch the client." => {
            "头显启动后，在尝试启动客户端之前等待的秒数。"
        }
        "UDP: Faster, but less stable than TCP. Try this if your network is well optimized and free of interference. TCP: Slower than UDP, but more stable. Pick this if you experience video or audio stutters with UDP." => {
            "UDP：更快，但比 TCP 更不稳定。如果你的网络优化良好且干扰较少，可以尝试使用。TCP：比 UDP 慢，但更稳定。如果使用 UDP 时出现视频或音频卡顿，请选择 TCP。"
        }
        "Which release type of client should ALVR look for when establishing a wired connection." => {
            "建立有线连接时，ALVR 应查找哪一种发布类型的客户端。"
        }
        "Wether ALVR should try to automatically launch the client when establishing a wired connection." => {
            "建立有线连接时，ALVR 是否应尝试自动启动客户端。"
        }
        "If on_connect.bat exists alongside session.json, it will be run on headset connect. Env var ACTION will be set to `connect`." => {
            "如果 `session.json` 同目录下存在 `on_connect.bat`，则会在头显连接时运行。环境变量 `ACTION` 会被设为 `connect`。"
        }
        "If on_connect.sh exists alongside session.json, it will be run on headset connect. Env var ACTION will be set to `connect`." => {
            "如果 `session.json` 同目录下存在 `on_connect.sh`，则会在头显连接时运行。环境变量 `ACTION` 会被设为 `connect`。"
        }
        "If on_disconnect.bat exists alongside session.json, it will be run on headset disconnect. Env var ACTION will be set to `disconnect`." => {
            "如果 `session.json` 同目录下存在 `on_disconnect.bat`，则会在头显断开时运行。环境变量 `ACTION` 会被设为 `disconnect`。"
        }
        "If on_disconnect.sh exists alongside session.json, it will be run on headset disconnect. Env var ACTION will be set to `disconnect`." => {
            "如果 `session.json` 同目录下存在 `on_disconnect.sh`，则会在头显断开时运行。环境变量 `ACTION` 会被设为 `disconnect`。"
        }
        "Allow cross-origin browser requests to control ALVR settings remotely." => {
            "允许跨域浏览器请求远程控制 ALVR 设置。"
        }
        "If the client, server or the network discarded one packet, discard packets until a IDR packet is found." => {
            "如果客户端、服务器或网络丢弃了一个数据包，则会持续丢弃后续数据包，直到找到一个 IDR 数据包。"
        }
        "The server discards video packets if it can't push them to the network. This could happen on TCP. A IDR frame is requested in this case." => {
            "如果服务器无法将视频数据包推送到网络中，它会丢弃这些数据包。这种情况可能发生在 TCP 模式下。此时会请求一个 IDR 帧。"
        }
        "Notification tips teach you how to use ALVR" => "通知提示会教你如何使用 ALVR。",
        "This applies only to certain error or warning messages." => {
            "这仅适用于某些错误或警告消息。"
        }
        "Write logs into the session_log.txt file." => "将日志写入 `session_log.txt` 文件。",
        "These settings enable extra spammy logs for debugging purposes." => {
            "这些设置会启用额外的高频日志，用于调试。"
        }
        "Launches SteamVR automatically when the ALVR dashboard is opened, and closes it when the dashboard is closed." => {
            "当打开 ALVR 控制面板时自动启动 SteamVR，并在关闭控制面板时自动关闭 SteamVR。"
        }
        "Directly start the VR server, bypassing Steam. Will run start_server.bat if it exists alongside session.json, and try to automatically find SteamVR otherwise." => {
            "直接启动 VR 服务器，绕过 Steam。如果 `session.json` 同目录下存在 `start_server.bat`，则会运行它；否则会尝试自动查找 SteamVR。"
        }
        "Directly start the VR server, bypassing Steam. Will run start_server.sh if it exists alongside session.json, and try to automatically find SteamVR otherwise." => {
            "直接启动 VR 服务器，绕过 Steam。如果 `session.json` 同目录下存在 `start_server.sh`，则会运行它；否则会尝试自动查找 SteamVR。"
        }
        "Async Compute is currently broken in SteamVR, keep disabled. ONLY FOR TESTING." => {
            "SteamVR 中的 Async Compute 当前存在问题，请保持禁用。仅用于测试。"
        }
        "Async reprojection only works if you can always hit at least half of your refresh rate." => {
            "异步重投影仅在你始终至少达到一半刷新率时才有效。"
        }
        "Linear and angular velocity multiplier for debug purposes. It does not update in real time." => {
            "用于调试的线速度和角速度乘数。该设置不会实时更新。"
        }
        _ => return None,
    })
}

static CURRENT_LANGUAGE: AtomicU8 = AtomicU8::new(Language::English as u8);

pub fn current_language() -> Language {
    match CURRENT_LANGUAGE.load(Ordering::Relaxed) {
        1 => Language::Chinese,
        _ => Language::English,
    }
}

pub fn set_current_language(language: Language) {
    CURRENT_LANGUAGE.store(language as u8, Ordering::Relaxed);
}

pub fn tr<'a>(text: &'a str) -> Cow<'a, str> {
    current_language().translate(text)
}
