import SwiftUI

/// Presentation UI only; connect isPlaying to your audio engine.
struct MiniPlayer: View {
    var title = "Soft Focus"
    var artist = "Sunday Sessions"
    @Binding var isPlaying: Bool
    @State private var expanded = false
    @Namespace private var artworkSpace
    @Environment(\.accessibilityReduceMotion) private var reduceMotion

    var body: some View {
        VStack(spacing: 24) {
            if expanded {
                Button { toggle() } label: {
                    Capsule().fill(.secondary.opacity(0.3)).frame(width: 36, height: 4).padding(12)
                }.accessibilityLabel("Collapse player")
                albumCover.frame(width: 248, height: 248)
                    .matchedGeometryEffect(id: "cover", in: artworkSpace)
                VStack(spacing: 7) {
                    Text(title).font(.title2.bold())
                    Text(artist).font(.subheadline).foregroundStyle(.secondary)
                }
                HStack(spacing: 12) {
                    Image(systemName: "waveform")
                    Text(isPlaying ? "Now playing" : "Paused")
                }.font(.caption).foregroundStyle(.secondary)
                playButton.font(.system(size: 34)).padding(.bottom, 18)
            } else {
                HStack(spacing: 14) {
                    Button { toggle() } label: {
                        HStack(spacing: 14) {
                            albumCover.frame(width: 48, height: 48)
                                .matchedGeometryEffect(id: "cover", in: artworkSpace)
                            VStack(alignment: .leading, spacing: 4) {
                                Text(title).font(.subheadline.weight(.semibold))
                                Text(artist).font(.caption).foregroundStyle(.secondary)
                            }
                            Spacer(minLength: 0)
                        }.contentShape(Rectangle())
                    }.accessibilityLabel("Expand \(title) player")
                    playButton.font(.title3).padding(.trailing, 8)
                }.padding(12)
            }
        }
        .frame(maxWidth: 340)
        .background(Color(.secondarySystemGroupedBackground), in: RoundedRectangle(cornerRadius: expanded ? 32 : 22))
        .overlay(RoundedRectangle(cornerRadius: expanded ? 32 : 22).stroke(.primary.opacity(0.06)))
        .shadow(color: .black.opacity(0.08), radius: 24, y: 12)
        .buttonStyle(.plain)
    }

    private var albumCover: some View {
        GeometryReader { proxy in
            ZStack {
                Color(red: 0.81, green: 0.39, blue: 0.23)
                Circle().fill(Color(red: 0.97, green: 0.71, blue: 0.42))
                    .frame(width: proxy.size.width * 0.76).offset(x: proxy.size.width * 0.19, y: -proxy.size.width * 0.08)
                Rectangle().fill(Color(red: 0.29, green: 0.38, blue: 0.32))
                    .frame(height: proxy.size.height * 0.44).rotationEffect(.degrees(-22)).offset(y: proxy.size.height * 0.42)
            }.clipShape(RoundedRectangle(cornerRadius: proxy.size.width * 0.08))
        }.accessibilityHidden(true)
    }

    private var playButton: some View {
        Button { isPlaying.toggle() } label: {
            Image(systemName: isPlaying ? "pause.fill" : "play.fill").frame(width: 44, height: 44)
        }.accessibilityLabel(isPlaying ? "Pause" : "Play")
    }
    private func toggle() {
        withAnimation(reduceMotion ? nil : .spring(response: 0.52, dampingFraction: 0.86)) { expanded.toggle() }
    }
}

struct MiniPlayerDemo: View {
    @State private var isPlaying = false
    var body: some View {
        VStack(spacing: 36) {
            Text("A soundtrack for slowing down.").font(.title3.weight(.medium))
            Spacer(minLength: 0)
            MiniPlayer(isPlaying: $isPlaying)
        }.padding(24).frame(maxWidth: .infinity, maxHeight: .infinity)
            .background(Color(.systemGroupedBackground))
    }
}

#Preview { MiniPlayerDemo() }
