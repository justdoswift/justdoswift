import { craftedComponents } from "./crafted-components";

export type ComponentCategory = "Motion" | "Controls" | "Navigation" | "Surfaces";

export type SwiftComponent = {
  slug: string;
  title: string;
  eyebrow: string;
  description: string;
  longDescription: string;
  category: ComponentCategory;
  ios: string;
  swift: string;
  accent: string;
  variant: "swap" | "toggle" | "card" | "dock" | "toast" | "sheet" | "video";
  access: "Free" | "Pro";
  featured?: boolean;
  source: string;
  usage: string;
  video?: string;
  poster?: string;
  videoKind?: "remotion" | "swiftui";
  demo?: string;
};

export const swiftComponents: SwiftComponent[] = [
  ...craftedComponents,
  {
    slug: "action-swap",
    title: "Action Swap",
    eyebrow: "Delightful feedback",
    description: "在图标与文案之间丝滑交换，让每一次操作都有明确反馈。",
    longDescription:
      "一个为复制、收藏和完成状态设计的轻量按钮。内容在同一空间内完成模糊、缩放与位移，保持布局稳定，也完整支持 Reduce Motion。",
    category: "Motion",
    ios: "iOS 17+",
    swift: "Swift 6",
    accent: "#8ea6ff",
    variant: "swap",
    access: "Free",
    featured: true,
    usage: `ActionSwapButton(
    idleTitle: "Copy code",
    successTitle: "Copied"
) {
    UIPasteboard.general.string = source
}`,
    source: `import SwiftUI

struct ActionSwapButton: View {
    let idleTitle: String
    let successTitle: String
    let action: () -> Void

    @Environment(\\.accessibilityReduceMotion) private var reduceMotion
    @State private var isComplete = false

    var body: some View {
        Button {
            action()

            withAnimation(reduceMotion ? nil : .spring(
                response: 0.34,
                dampingFraction: 0.72
            )) {
                isComplete = true
            }

            Task {
                try? await Task.sleep(for: .seconds(1.5))
                withAnimation(.easeOut(duration: 0.2)) {
                    isComplete = false
                }
            }
        } label: {
            HStack(spacing: 8) {
                Image(systemName: isComplete ? "checkmark" : "doc.on.doc")
                    .contentTransition(.symbolEffect(.replace))

                Text(isComplete ? successTitle : idleTitle)
                    .contentTransition(.numericText())
            }
            .font(.system(.subheadline, design: .rounded, weight: .semibold))
            .foregroundStyle(.black)
            .padding(.horizontal, 18)
            .frame(height: 46)
            .background(.white, in: Capsule())
        }
        .buttonStyle(.plain)
        .accessibilityLabel(isComplete ? successTitle : idleTitle)
    }
}`,
  },
  {
    slug: "liquid-toggle",
    title: "Liquid Toggle",
    eyebrow: "Matched geometry",
    description: "带有液态跟随感的双态选择器，适合视图、主题与模式切换。",
    longDescription:
      "用 matchedGeometryEffect 保持选中背景的空间连续性，并把触觉反馈限制在真实的状态变化上。紧凑、稳定，适合导航栏和工具面板。",
    category: "Controls",
    ios: "iOS 17+",
    swift: "Swift 6",
    accent: "#b9f08a",
    variant: "toggle",
    access: "Free",
    usage: `LiquidToggle(
    options: ["Design", "Code"],
    selection: $selection
)`,
    source: `import SwiftUI

struct LiquidToggle: View {
    let options: [String]
    @Binding var selection: String
    @Namespace private var selectionSpace

    var body: some View {
        HStack(spacing: 4) {
            ForEach(options, id: \\.self) { option in
                Button {
                    guard selection != option else { return }
                    withAnimation(.spring(response: 0.38, dampingFraction: 0.78)) {
                        selection = option
                    }
                } label: {
                    Text(option)
                        .font(.footnote.weight(.semibold))
                        .foregroundStyle(selection == option ? .black : .secondary)
                        .padding(.horizontal, 16)
                        .frame(height: 36)
                        .background {
                            if selection == option {
                                Capsule()
                                    .fill(.white)
                                    .matchedGeometryEffect(id: "selection", in: selectionSpace)
                            }
                        }
                }
                .buttonStyle(.plain)
            }
        }
        .padding(4)
        .background(.ultraThinMaterial, in: Capsule())
        .sensoryFeedback(.selection, trigger: selection)
    }
}`,
  },
  {
    slug: "spatial-card",
    title: "Spatial Card",
    eyebrow: "Depth & light",
    description: "响应拖动与指针位置的空间卡片，用微妙光泽表达层级。",
    longDescription:
      "通过有限角度的 3D 旋转、动态高光和回弹阻尼，构建有触感但不过分的空间表面。适合订阅方案、钱包与精选内容。",
    category: "Surfaces",
    ios: "iOS 17+",
    swift: "Swift 6",
    accent: "#f3a6ff",
    variant: "card",
    access: "Pro",
    usage: `SpatialCard {
    PremiumPlanView()
}
.frame(width: 320, height: 190)`,
    source: `import SwiftUI

struct SpatialCard<Content: View>: View {
    @ViewBuilder var content: Content
    @State private var offset: CGSize = .zero

    var body: some View {
        content
            .padding(24)
            .frame(maxWidth: .infinity, maxHeight: .infinity)
            .background {
                RoundedRectangle(cornerRadius: 28, style: .continuous)
                    .fill(.ultraThinMaterial)
                    .overlay(alignment: .topLeading) {
                        Circle()
                            .fill(.white.opacity(0.32))
                            .frame(width: 180)
                            .blur(radius: 34)
                            .offset(x: offset.width - 80, y: offset.height - 90)
                    }
            }
            .clipShape(RoundedRectangle(cornerRadius: 28, style: .continuous))
            .rotation3DEffect(.degrees(offset.height / 20), axis: (x: 1, y: 0, z: 0))
            .rotation3DEffect(.degrees(-offset.width / 20), axis: (x: 0, y: 1, z: 0))
            .gesture(
                DragGesture(minimumDistance: 0)
                    .onChanged { value in
                        offset = CGSize(
                            width: min(24, max(-24, value.translation.width)),
                            height: min(24, max(-24, value.translation.height))
                        )
                    }
                    .onEnded { _ in
                        withAnimation(.spring(response: 0.42, dampingFraction: 0.66)) {
                            offset = .zero
                        }
                    }
            )
    }
}`,
  },
  {
    slug: "morphing-dock",
    title: "Morphing Dock",
    eyebrow: "Adaptive navigation",
    description: "会呼吸的底部导航，选中项自然扩张并携带当前上下文。",
    longDescription:
      "选中状态以共享几何背景在项目之间移动，同时让标签按需出现。它比传统 Tab Bar 更有表现力，但仍保留稳定的点击目标和清晰的可访问标签。",
    category: "Navigation",
    ios: "iOS 17+",
    swift: "Swift 6",
    accent: "#73d9ff",
    variant: "dock",
    access: "Pro",
    usage: `MorphingDock(
    items: DockItem.defaults,
    selection: $selection
)`,
    source: `import SwiftUI

struct DockItem: Identifiable, Hashable {
    let id: String
    let title: String
    let icon: String
}

struct MorphingDock: View {
    let items: [DockItem]
    @Binding var selection: DockItem
    @Namespace private var dockSpace

    var body: some View {
        HStack(spacing: 6) {
            ForEach(items) { item in
                Button {
                    withAnimation(.spring(response: 0.42, dampingFraction: 0.76)) {
                        selection = item
                    }
                } label: {
                    HStack(spacing: 7) {
                        Image(systemName: item.icon)
                        if selection == item {
                            Text(item.title)
                                .transition(.blurReplace.combined(with: .opacity))
                        }
                    }
                    .font(.subheadline.weight(.semibold))
                    .padding(.horizontal, selection == item ? 15 : 12)
                    .frame(height: 42)
                    .background {
                        if selection == item {
                            Capsule()
                                .fill(.white)
                                .matchedGeometryEffect(id: "active", in: dockSpace)
                        }
                    }
                    .foregroundStyle(selection == item ? .black : .secondary)
                }
                .buttonStyle(.plain)
                .accessibilityLabel(item.title)
            }
        }
        .padding(6)
        .background(.ultraThinMaterial, in: Capsule())
    }
}`,
  },
  {
    slug: "toast-stack",
    title: "Toast Stack",
    eyebrow: "Status in motion",
    description: "多条通知以卡片栈进入、聚拢与离开，不打断当前任务。",
    longDescription:
      "通知会在统一锚点上形成具有深度的堆栈，最新状态保持可读，其余内容仅提供上下文。自动消失、手势关闭和 Reduce Motion 都在同一状态模型中完成。",
    category: "Surfaces",
    ios: "iOS 17+",
    swift: "Swift 6",
    accent: "#ffbd72",
    variant: "toast",
    access: "Free",
    usage: `ToastStack(messages: messages)
    .animation(
        .spring(response: 0.45, dampingFraction: 0.78),
        value: messages
    )`,
    source: `import SwiftUI

struct ToastMessage: Identifiable, Equatable {
    let id = UUID()
    let title: String
    let symbol: String
}

struct ToastStack: View {
    let messages: [ToastMessage]

    var body: some View {
        ZStack(alignment: .top) {
            ForEach(Array(messages.prefix(3).enumerated()), id: \\.element.id) { index, message in
                HStack(spacing: 12) {
                    Image(systemName: message.symbol)
                        .frame(width: 28, height: 28)
                        .background(.white.opacity(0.12), in: Circle())
                    Text(message.title)
                        .font(.subheadline.weight(.medium))
                    Spacer(minLength: 12)
                }
                .padding(14)
                .background(.ultraThinMaterial, in: RoundedRectangle(cornerRadius: 18))
                .scaleEffect(1 - CGFloat(index) * 0.055)
                .offset(y: CGFloat(index) * 12)
                .zIndex(Double(messages.count - index))
                .transition(.move(edge: .top).combined(with: .opacity))
            }
        }
        .animation(.spring(response: 0.45, dampingFraction: 0.78), value: messages)
        .accessibilityElement(children: .contain)
    }
}`,
  },
  {
    slug: "spring-sheet",
    title: "Spring Sheet",
    eyebrow: "Layered presentation",
    description: "从触发控件自然生长的操作面板，兼顾速度、层级与手感。",
    longDescription:
      "一个适合快捷操作和轻量编辑的原生 Sheet 配方。它使用系统 detents、背景交互与圆角，保留平台行为，同时让进入和退出更加连贯。",
    category: "Motion",
    ios: "iOS 17+",
    swift: "Swift 6",
    accent: "#ff8fa3",
    variant: "sheet",
    access: "Free",
    usage: `.sheet(isPresented: $isPresented) {
    SpringSheetContent()
        .presentationDetents([.height(280), .medium])
}`, 
    source: `import SwiftUI

struct SpringSheetDemo: View {
    @State private var isPresented = false

    var body: some View {
        Button("Open actions") {
            isPresented = true
        }
        .buttonStyle(.borderedProminent)
        .sheet(isPresented: $isPresented) {
            VStack(alignment: .leading, spacing: 20) {
                Capsule()
                    .fill(.secondary.opacity(0.35))
                    .frame(width: 36, height: 5)
                    .frame(maxWidth: .infinity)

                Text("Quick actions")
                    .font(.title2.bold())

                ForEach(["Duplicate", "Share", "Archive"], id: \\.self) { title in
                    Button(title) { }
                        .buttonStyle(.plain)
                        .frame(maxWidth: .infinity, alignment: .leading)
                }

                Spacer()
            }
            .padding(24)
            .presentationDetents([.height(280), .medium])
            .presentationCornerRadius(30)
            .presentationBackgroundInteraction(.enabled(upThrough: .medium))
        }
    }
}`,
  },
];

export const categories = ["All", "Motion", "Controls", "Navigation", "Surfaces"] as const;

export function getComponent(slug: string) {
  return swiftComponents.find((component) => component.slug === slug);
}
