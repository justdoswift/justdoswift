import SwiftUI

/// Replace the content builder with Image(...).resizable().scaledToFill().
struct PhotoStack<Item: Identifiable, Content: View>: View {
    let items: [Item]
    @ViewBuilder let content: (Item) -> Content
    @Environment(\.accessibilityReduceMotion) private var reduceMotion
    @State private var expanded = false
    @State private var selected = 0
    @State private var drag = CGSize.zero

    private var count: Int { min(items.count, 3) }

    var body: some View {
        VStack(spacing: 24) {
            ZStack {
                ForEach(Array(items.prefix(3).enumerated()), id: \.element.id) { index, item in
                    let relative = count == 0 ? 0 : (index - selected + count) % count
                    content(item)
                        .frame(width: expanded ? 144 : 208, height: expanded ? 210 : 250)
                        .clipShape(RoundedRectangle(cornerRadius: 20))
                        .overlay(RoundedRectangle(cornerRadius: 20).stroke(.white, lineWidth: 5))
                        .shadow(color: .black.opacity(0.12), radius: 16, y: 10)
                        .rotationEffect(.degrees(reduceMotion ? 0 : angle(relative)))
                        .offset(x: expanded ? (CGFloat(index) - CGFloat(count - 1) / 2) * 86 : CGFloat(relative) * 12,
                                y: expanded ? (index == 1 ? -12 : 8) : CGFloat(relative) * -7)
                        .offset(relative == 0 && !expanded ? drag : .zero)
                        .zIndex(expanded ? Double(index) : Double(count - relative))
                        .accessibilityHidden(relative != 0 && !expanded)
                        .onTapGesture { toggle() }
                }
            }
            .frame(maxWidth: .infinity).frame(height: 310)
            .contentShape(Rectangle())
            .gesture(DragGesture(minimumDistance: 12)
                .onChanged { value in
                    guard !expanded else { return }
                    drag = CGSize(width: value.translation.width * 0.55, height: 0)
                }
                .onEnded { value in
                    withAnimation(motion) {
                        if abs(value.translation.width) > 55 && count > 0 { selected = (selected + 1) % count }
                        drag = .zero
                    }
                })
            HStack(spacing: 20) {
                Button(expanded ? "Gather memories" : "Explore the stack", action: toggle)
                if count > 1 {
                    Button { withAnimation(motion) { selected = (selected + 1) % count } }
                        label: { Image(systemName: "arrow.right") }
                        .accessibilityLabel("Next photo")
                }
            }.font(.footnote.weight(.medium)).tint(.primary)
        }
        .onChange(of: items.map(\.id)) { _, _ in selected = 0; drag = .zero }
    }

    private var motion: Animation? { reduceMotion ? nil : .spring(response: 0.5, dampingFraction: 0.78) }
    private func angle(_ index: Int) -> Double { expanded ? 0 : [0.0, -9, 8][index] }
    private func toggle() { withAnimation(motion) { expanded.toggle(); drag = .zero } }
}

struct TravelPostcard: Identifiable {
    let id: Int
    let name: String
    let subtitle: String
    let sky: Color
    let land: Color
}

/// Original procedural artwork: the demo needs no downloaded image assets.
struct PostcardArtwork: View {
    let card: TravelPostcard
    var body: some View {
        GeometryReader { proxy in
            ZStack(alignment: .bottomLeading) {
                card.sky
                Circle().fill(Color(red: 1, green: 0.88, blue: 0.61))
                    .frame(width: proxy.size.width * 0.33, height: proxy.size.width * 0.33)
                    .offset(x: proxy.size.width * 0.48, y: -proxy.size.height * 0.54)
                Path { path in
                    let w = proxy.size.width, h = proxy.size.height
                    path.move(to: CGPoint(x: 0, y: h * 0.72))
                    path.addQuadCurve(to: CGPoint(x: w, y: h * 0.58), control: CGPoint(x: w * 0.42, y: h * 0.1))
                    path.addLine(to: CGPoint(x: w, y: h)); path.addLine(to: CGPoint(x: 0, y: h))
                }.fill(card.land)
                VStack(alignment: .leading, spacing: 6) {
                    Text(card.name).font(.system(size: proxy.size.width * 0.106, weight: .bold, design: .serif))
                    Text(card.subtitle).font(.system(size: proxy.size.width * 0.043, weight: .medium)).tracking(1)
                }.foregroundStyle(.white).padding(proxy.size.width * 0.1)
            }
        }
    }
}

struct PhotoStackDemo: View {
    private let cards = [
        TravelPostcard(id: 0, name: "Alpine", subtitle: "SLOW WEEKENDS", sky: Color(red: 0.75, green: 0.85, blue: 0.82), land: Color(red: 0.25, green: 0.43, blue: 0.37)),
        TravelPostcard(id: 1, name: "Coast", subtitle: "TAKE THE LONG WAY", sky: Color(red: 0.69, green: 0.82, blue: 0.86), land: Color(red: 0.32, green: 0.51, blue: 0.61)),
        TravelPostcard(id: 2, name: "Dune", subtitle: "NOWHERE TO RUSH", sky: Color(red: 0.9, green: 0.75, blue: 0.61), land: Color(red: 0.67, green: 0.39, blue: 0.27))
    ]
    var body: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Somewhere, slowly.").font(.title2.weight(.semibold)).padding(.horizontal, 28)
            Text("Three places worth keeping.").foregroundStyle(.secondary).padding(.horizontal, 28)
            PhotoStack(items: cards) { PostcardArtwork(card: $0) }
        }.frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

#Preview { PhotoStackDemo() }
