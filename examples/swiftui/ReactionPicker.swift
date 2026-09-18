import SwiftUI

struct ReactionOption: Identifiable, Equatable {
    let id: String
    let emoji: String
    let label: String
    static let defaults = [
        ReactionOption(id: "love", emoji: "❤️", label: "Love"),
        ReactionOption(id: "celebrate", emoji: "🎉", label: "Celebrate"),
        ReactionOption(id: "laugh", emoji: "😂", label: "Laugh"),
        ReactionOption(id: "surprise", emoji: "😮", label: "Surprise"),
        ReactionOption(id: "like", emoji: "👍", label: "Like")
    ]
}

struct ReactionPicker: View {
    @Binding var selection: ReactionOption?
    var options = ReactionOption.defaults
    @Environment(\.accessibilityReduceMotion) private var reduceMotion
    @State private var open = false
    @State private var highlighted = 0

    var body: some View {
        HStack {
            Label(selection?.emoji ?? "React", systemImage: "face.smiling")
                .font(.subheadline.weight(.medium))
                .padding(.horizontal, 16).frame(height: 44)
                .background(Color(.secondarySystemGroupedBackground), in: Capsule())
                .overlay(Capsule().stroke(.primary.opacity(0.07)))
                .contentShape(Capsule())
                .gesture(LongPressGesture(minimumDuration: 0.3, maximumDistance: 25)
                    .sequenced(before: DragGesture(minimumDistance: 0))
                    .onChanged { value in
                        switch value {
                        case .second(true, let drag):
                            withAnimation(motion) { open = true }
                            if let drag, !options.isEmpty {
                                highlighted = min(options.count - 1, max(0, Int((drag.translation.width + 20) / 50)))
                            }
                        default: break
                        }
                    }
                    .onEnded { value in
                        if case .second(true, _) = value, options.indices.contains(highlighted) {
                            selection = options[highlighted]
                        }
                        withAnimation(motion) { open = false }
                        highlighted = 0
                    })
                .onTapGesture {
                    if selection != nil { selection = nil }
                }
                .accessibilityElement(children: .ignore)
                .accessibilityLabel(selection.map { "Reaction: \($0.label)" } ?? "Choose a reaction")
                .accessibilityAddTraits(.isButton)
                .accessibilityActions {
                    if selection != nil {
                        Button("Remove reaction") { selection = nil }
                    }
                    ForEach(options) { option in
                        Button(option.label) { selection = option }
                    }
                }
                .overlay(alignment: .bottomLeading) {
                    if open {
                        HStack(spacing: 0) {
                            ForEach(Array(options.enumerated()), id: \.element.id) { index, option in
                                Text(option.emoji).font(.system(size: 27))
                                    .frame(width: 50, height: 54)
                                    .scaleEffect(index == highlighted && !reduceMotion ? 1.38 : 1)
                                    .offset(y: index == highlighted && !reduceMotion ? -12 : 0)
                                    .animation(motion, value: highlighted)
                            }
                        }
                        .padding(6).background(.regularMaterial, in: Capsule())
                        .shadow(color: .black.opacity(0.12), radius: 18, y: 8)
                        .fixedSize().offset(y: -62)
                        .transition(reduceMotion ? .opacity : .scale(scale: 0.85, anchor: .bottomLeading).combined(with: .opacity))
                        .allowsHitTesting(false)
                    }
                }
            Spacer(minLength: 0)
        }
        .sensoryFeedback(.selection, trigger: highlighted)
        .onDisappear { open = false; highlighted = 0 }
    }
    private var motion: Animation? { reduceMotion ? nil : .spring(response: 0.28, dampingFraction: 0.68) }
}

struct ReactionPickerDemo: View {
    @State private var selection: ReactionOption?
    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            HStack {
                Text("A").font(.headline).frame(width: 40, height: 40).background(.orange.opacity(0.15), in: Circle())
                VStack(alignment: .leading) { Text("Alex").bold(); Text("Just now").font(.caption).foregroundStyle(.secondary) }
            }
            Text("Made time for the things I love.").font(.title3).padding(.bottom, 60)
            ReactionPicker(selection: $selection)
            Text(selection.map { "\($0.label). Tap to remove." } ?? "Hold, slide, release.")
                .font(.caption).foregroundStyle(.secondary)
        }.padding(28).frame(maxWidth: 360)
            .background(Color(.systemGroupedBackground), in: RoundedRectangle(cornerRadius: 28))
            .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

#Preview { ReactionPickerDemo() }
