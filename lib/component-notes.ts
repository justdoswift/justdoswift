export const componentNotes: Record<string, { title: string; text: string }[]> = {
  "photo-stack": [
    { title: "A small stack, a shared rhythm", text: "叠放状态通过旋转、偏移与层级表达纸张感，展开后呈现前三张卡片。左右拖动翻到下一张，数据变化时重置索引。" },
    { title: "Bring your own photos", text: "PhotoStack 接受 Identifiable 数据和 ViewBuilder。演示使用代码绘制的原创旅行插画，无需外部图片；可替换为 Image 或 AsyncImage。" },
    { title: "Tune the feel", text: "调整卡片宽高、展开间距和 spring 的 response / dampingFraction。Reduce Motion 下关闭旋转与弹簧动画。" },
  ],
  "hold-to-confirm": [
    { title: "Commit only after a complete hold", text: "ContinuousClock 驱动进度。提前松手或手指偏离 28 点会取消，达到阈值仅执行一次回调。离开页面时取消任务。" },
    { title: "A confirmation everyone can use", text: "VoiceOver 可以直接激活确认，不要求持续按住。根据实际业务选择操作是否需要二次确认，不应把长按当成唯一保护。" },
    { title: "Make it yours", text: "title 控制提示，duration 控制长按时长，tint 控制色调。演示默认为 1.2 秒，完成后保留反馈 1.6 秒。" },
  ],
  "mini-player": [
    { title: "Keep the artwork continuous", text: "收起与展开共用 matchedGeometryEffect 标识。封面位置与尺寸连续变化，收起按钮与播放按钮保持独立。" },
    { title: "Your audio engine, your state", text: "isPlaying 是双向绑定。这是播放器界面组件，不包含音频下载、播放引擎或后台音频配置。Remotion 视频同样没有配乐。" },
    { title: "Adapt the presentation", text: "通过 title、artist 和 isPlaying 接入你的内容。替换 albumCover 可使用本地专辑图片；Reduce Motion 下关闭展开动画。" },
  ],
  "reaction-picker": [
    { title: "Preview before you commit", text: "LongPressGesture 与 DragGesture 串联。滑动只改变 highlighted，松手后才更新 selection，避免中途修改真实选择。" },
    { title: "Accessible reactions", text: "默认提供五种表情，每种都有文字标签和独立无障碍操作。可通过 options 传入自己的 ReactionOption 数组。" },
    { title: "Leave room above the trigger", text: "表情托盘向上展开，接入列表时要预留顶部空间，避免被父视图裁切。可调整 0.3 秒长按门槛和 50 点选项间距。" },
  ],
  "send-button": [
    { title: "A real async boundary", text: "send 接受 async throws 回调。发送中和成功反馈期间禁用重复点击；失败显示 Try again，不会清空输入内容。" },
    { title: "Test the unhappy path", text: "Demo 自带 Simulate a failed request 开关。生产接入时用真实网络请求替换 Task.sleep，并让宿主应用在成功后决定是否清空草稿。" },
    { title: "Cancel with the view", text: "离开页面时取消 Task。回调结束后再次检查取消状态，避免离开后仍显示成功。enabled 可以绑定输入有效性。" },
  ],
  "pull-to-search": [
    { title: "Let the scroll reveal search", text: "PreferenceKey 读取 ScrollView 顶部位移。超过 threshold 后显示搜索框并请求焦点，默认门槛为 95 点。" },
    { title: "Keep a direct route", text: "标题栏的搜索按钮提供不依赖手势的入口。清空、取消、键盘收起和无结果界面在 Demo 中都有示例。" },
    { title: "Own the filtering", text: "query 是双向绑定，content 接收你筛选后的行。网络搜索请在宿主中添加防抖与取消，不要直接对每一帧滚动发送请求。" },
  ],
  "action-swap": [
    { title: "State-driven feedback", text: "用一个状态控制文案与图标，让操作结果在原位呈现。复制成功后短暂停留，再回到初始状态。" },
    { title: "A small, quick spring", text: "用短促的弹簧提供反馈。Swift 示例通过 accessibilityReduceMotion 读取系统偏好，减少主要状态变化的动效。" },
  ],
  "liquid-toggle": [
    { title: "One shared background", text: "选中背景使用同一个 matchedGeometryEffect 标识，在选项之间移动。绑定状态让它方便地接入父视图。" },
    { title: "Feedback on change", text: "只有选项真正变化时才触发 sensoryFeedback，重复点击当前选项不会产生多余反馈。" },
  ],
  "spatial-card": [
    { title: "Keep the angle small", text: "拖动位移被限制在一个小范围内，再映射为双轴旋转，避免卡片内容在交互过程中难以阅读。" },
    { title: "Light follows movement", text: "模糊高光和卡片朝向共同响应拖动，松手后使用弹簧回到初始位置。" },
  ],
  "morphing-dock": [
    { title: "Selection carries context", text: "仅为当前选项显示文字，其余选项保留图标。共享几何背景把切换过程连成一次连续移动。" },
    { title: "Keep labels available", text: "即使视觉上隐藏了文字，accessibilityLabel 仍保留每个导航项目的名称。" },
  ],
  "toast-stack": [
    { title: "A simple visual hierarchy", text: "最新通知位于最上层，后面的卡片通过缩放和位移形成堆叠。示例最多显示三条通知。" },
    { title: "Drive it with your data", text: "将消息数组作为状态传入，在你的应用中决定何时加入或移除。示例本身不处理消息队列和自动过期。" },
  ],
  "spring-sheet": [
    { title: "Use the system presentation", text: "SwiftUI 示例采用原生 sheet 与 presentationDetents，在固定高度和中等高度之间调整。" },
    { title: "Add your own actions", text: "Duplicate、Share 和 Archive 展示布局位置。在你的项目里补上对应的业务操作。" },
  ],
};
