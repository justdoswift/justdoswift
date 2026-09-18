import type { SwiftComponent } from "./components";
import { swiftSources } from "./generated/swift-sources";

export const craftedComponents: SwiftComponent[] = [
  {
    slug: "photo-stack", title: "Photo Stack", eyebrow: "Somewhere, slowly",
    description: "把回忆叠成一小摞。轻点展开，滑动翻阅，让照片拥有一点纸张的温度。",
    longDescription: "以旅行明信片为例，展示叠放、展开和翻阅三种状态。组件接收任意 Identifiable 数据与视图内容，可替换为你自己的照片。",
    category: "Surfaces", ios: "iOS 17+", swift: "Swift 6", accent: "#688b77", variant: "video", access: "Free", featured: true,
    source: swiftSources.PhotoStack, usage: "PhotoStackDemo()\n\n// Or bring your own data and images:\n// PhotoStack(items: photos) { photo in\n//     Image(photo.assetName).resizable().scaledToFill()\n// }",
    video: "/previews/photo-stack.mp4", poster: "/previews/photo-stack.png", videoKind: "remotion", demo: "PhotoStackDemo",
  },
  {
    slug: "hold-to-confirm", title: "Hold to Confirm", eyebrow: "Make it intentional",
    description: "用一次有分寸的长按确认操作。提前松手安全回退，完成后给出清晰反馈。",
    longDescription: "长按进度、取消和确认组成一个小而完整的状态机。用于归档等有意图的操作，并提供无需长按的 VoiceOver 替代动作。",
    category: "Controls", ios: "iOS 17+", swift: "Swift 6", accent: "#517860", variant: "video", access: "Free", featured: true,
    source: swiftSources.HoldToConfirm, usage: "HoldToConfirmDemo()\n\n// In your own view:\nHoldToConfirmButton(title: \"Hold to archive\", duration: 1.2) {\n    // Archive the selected item.\n}",
    video: "/previews/hold-to-confirm.mp4", poster: "/previews/hold-to-confirm.png", videoKind: "remotion", demo: "HoldToConfirmDemo",
  },
  {
    slug: "mini-player", title: "Mini Player", eyebrow: "Stay in the moment",
    description: "从轻巧的播放条展开到完整封面，播放状态和视觉焦点始终连续。",
    longDescription: "通过 matchedGeometryEffect 保持封面连续。提供展开、收起与播放状态绑定；音频引擎由宿主应用接入，示例不播放音频。",
    category: "Navigation", ios: "iOS 17+", swift: "Swift 6", accent: "#c48258", variant: "video", access: "Free",
    source: swiftSources.MiniPlayer, usage: "MiniPlayerDemo()\n\n// In a view with @State private var playing = false:\n// MiniPlayer(title: \"Soft Focus\", artist: \"Sunday Sessions\",\n//            isPlaying: $playing)",
    video: "/previews/mini-player.mp4", poster: "/previews/mini-player.png", videoKind: "remotion", demo: "MiniPlayerDemo",
  },
  {
    slug: "reaction-picker", title: "Reaction Picker", eyebrow: "A little more feeling",
    description: "长按唤出表情，滑过时轻轻放大，松手把心情留在原处。",
    longDescription: "长按与拖拽串联为一段连续手势。预选状态与最终选择分开管理，并为 VoiceOver 暴露每个表情的独立操作。",
    category: "Controls", ios: "iOS 17+", swift: "Swift 6", accent: "#c09c74", variant: "video", access: "Free",
    source: swiftSources.ReactionPicker, usage: "ReactionPickerDemo()\n\n// In a view with @State private var reaction: ReactionOption?:\n// ReactionPicker(selection: $reaction)",
    video: "/previews/reaction-picker.mp4", poster: "/previews/reaction-picker.png", videoKind: "remotion", demo: "ReactionPickerDemo",
  },
  {
    slug: "send-button", title: "Send Button", eyebrow: "A small hello",
    description: "从输入到发送，从等待到完成，把异步操作的每个状态交代清楚。",
    longDescription: "按钮拥有 idle、sending、sent、failed 四个状态。支持异步回调、重复提交保护、失败重试和离开页面时取消任务。",
    category: "Motion", ios: "iOS 17+", swift: "Swift 6", accent: "#6d8999", variant: "video", access: "Free",
    source: swiftSources.SendButton, usage: "SendButtonDemo()\n\n// Replace the delay with your own async request:\nSendButton {\n    try await Task.sleep(for: .seconds(1))\n}",
    video: "/previews/send-button.mp4", poster: "/previews/send-button.png", videoKind: "remotion", demo: "SendButtonDemo",
  },
  {
    slug: "pull-to-search", title: "Pull to Search", eyebrow: "Find your little things",
    description: "轻轻下拉，让搜索自然出现。找到收藏，也保留浏览时的清爽。",
    longDescription: "用滚动偏移触发搜索框，支持键盘焦点、清空、取消与空结果。顶部搜索按钮为不方便下拉的用户提供直接入口。",
    category: "Navigation", ios: "iOS 17+", swift: "Swift 6", accent: "#8e9674", variant: "video", access: "Free",
    source: swiftSources.PullToSearch, usage: "PullToSearchDemo()\n\n// Wrap your filtered rows with PullToSearch(query: $query) { … }",
    video: "/previews/pull-to-search.mp4", poster: "/previews/pull-to-search.png", videoKind: "remotion", demo: "PullToSearchDemo",
  },
];
