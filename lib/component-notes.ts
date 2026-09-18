export const componentNotes: Record<string, { title: string; text: string }[]> = {
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
