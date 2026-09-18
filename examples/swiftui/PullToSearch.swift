import SwiftUI

private struct PullSearchOffset: PreferenceKey {
    static let defaultValue: CGFloat = 0
    static func reduce(value: inout CGFloat, nextValue: () -> CGFloat) { value = nextValue() }
}

struct PullToSearch<Content: View>: View {
    @Binding var query: String
    var title = "Your collection"
    var threshold: CGFloat = 95
    @ViewBuilder let content: () -> Content
    @Environment(\.accessibilityReduceMotion) private var reduceMotion
    @FocusState private var focused: Bool
    @State private var visible = false
    @State private var offset: CGFloat = 0
    @State private var armed = true
    @State private var dragStartedAtTop: Bool?

    var body: some View {
        VStack(spacing: 0) {
            HStack {
                Text(title).font(.title2.bold())
                Spacer()
                Button { reveal() } label: { Image(systemName: "magnifyingglass").frame(width: 44, height: 44) }
                    .accessibilityLabel("Show search")
            }.padding(.horizontal, 22).padding(.vertical, 8)
            if visible {
                HStack(spacing: 12) {
                    HStack {
                        Image(systemName: "magnifyingglass").foregroundStyle(.secondary)
                        TextField("Search your collection", text: $query).focused($focused)
                            .submitLabel(.search).onSubmit { focused = false }
                        if !query.isEmpty {
                            Button { query = "" } label: { Image(systemName: "xmark.circle.fill") }
                                .accessibilityLabel("Clear search")
                        }
                    }.padding(13).background(.primary.opacity(0.05), in: RoundedRectangle(cornerRadius: 14))
                    Button("Cancel") {
                        focused = false; query = ""; armed = offset <= 2
                        withAnimation(motion) { visible = false }
                    }.font(.footnote)
                }.padding(.horizontal, 22).padding(.bottom, 14)
                    .transition(.opacity.combined(with: .move(edge: .top)))
            }
            ScrollView {
                VStack(spacing: 0) {
                    GeometryReader { proxy in
                        Color.clear.preference(key: PullSearchOffset.self, value: proxy.frame(in: .named("pull-search")).minY)
                    }.frame(height: 0)
                    content()
                }.frame(maxWidth: .infinity)
            }
            .coordinateSpace(name: "pull-search")
            .scrollBounceBehavior(.always)
            .scrollDismissesKeyboard(.interactively)
            .contentShape(Rectangle())
            .simultaneousGesture(DragGesture(minimumDistance: 12)
                .onChanged { value in
                    if dragStartedAtTop == nil { dragStartedAtTop = offset >= -2 }
                    // Use finger travel for the threshold: rubber-banding otherwise
                    // makes the required pull depend on screen size and content height.
                    if dragStartedAtTop == true && value.translation.height > max(40, threshold)
                        && armed && !visible {
                        armed = false
                        reveal()
                    }
                }
                .onEnded { _ in
                    dragStartedAtTop = nil
                    if offset <= 2 { armed = true }
                })
            .overlay(alignment: .top) {
                if !visible && offset > 8 {
                    Image(systemName: "magnifyingglass")
                        .font(.system(size: 18)).foregroundStyle(.secondary)
                        .opacity(min(offset / max(20, threshold), 1))
                        .offset(y: min(offset / 3, 28))
                        .allowsHitTesting(false).accessibilityHidden(true)
                }
            }
            .onPreferenceChange(PullSearchOffset.self) { value in
                offset = value
                if value <= 2 { armed = true }
                // Also support scrolling devices that do not produce a touch drag.
                if value > max(40, threshold) && armed && !visible {
                    armed = false
                    reveal()
                }
            }
        }.tint(.primary)
    }
    private var motion: Animation? { reduceMotion ? nil : .spring(response: 0.4, dampingFraction: 0.9) }
    private func reveal() {
        withAnimation(motion) { visible = true }
        focused = true
    }
}

struct PullToSearchDemo: View {
    @State private var query = ""
    private let items = ["Morning pages", "Weekend places", "Small ideas", "Things to make", "Books to revisit", "Slow Sundays"]
    private var filtered: [String] { items.filter { query.isEmpty || $0.localizedCaseInsensitiveContains(query) } }
    var body: some View {
        PullToSearch(query: $query, title: "Saved for later") {
            LazyVStack(spacing: 12) {
                if filtered.isEmpty {
                    ContentUnavailableView.search(text: query)
                } else {
                    ForEach(filtered, id: \.self) { item in
                        HStack(spacing: 15) {
                            Image(systemName: "bookmark").frame(width: 46, height: 46)
                                .background(.orange.opacity(0.1), in: RoundedRectangle(cornerRadius: 12))
                            VStack(alignment: .leading, spacing: 5) {
                                Text(item).font(.subheadline.weight(.medium))
                                Text("A little collection").font(.caption).foregroundStyle(.secondary)
                            }
                            Spacer()
                            Image(systemName: "chevron.right").font(.caption).foregroundStyle(.tertiary)
                        }.padding(16).background(.background, in: RoundedRectangle(cornerRadius: 18))
                    }
                }
            }.padding(22)
        }.background(Color(.systemGroupedBackground))
    }
}

#Preview { PullToSearchDemo() }
