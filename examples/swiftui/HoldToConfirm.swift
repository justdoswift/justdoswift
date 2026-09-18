import SwiftUI

/// iOS 17+. A cancellable hold, with an accessible alternative action.
@MainActor
struct HoldToConfirmButton: View {
    var title = "Hold to archive"
    var duration: Double = 1.2
    var tint: Color = Color(red: 0.24, green: 0.43, blue: 0.35)
    let onConfirm: () -> Void

    @Environment(\.accessibilityReduceMotion) private var reduceMotion
    @State private var progress = 0.0
    @State private var complete = false
    @State private var cancelled = false
    @State private var holdTask: Task<Void, Never>?

    var body: some View {
        ZStack(alignment: .leading) {
            Capsule().fill(tint.opacity(0.09))
            GeometryReader { proxy in
                Capsule().fill(tint.opacity(0.18))
                    .frame(width: proxy.size.width * progress)
            }.clipShape(Capsule())
            Label(complete ? "Archived" : title,
                  systemImage: complete ? "checkmark.circle.fill" : "archivebox")
                .font(.system(.subheadline, design: .rounded, weight: .semibold))
                .foregroundStyle(tint)
                .frame(maxWidth: .infinity)
        }
        .frame(width: 240, height: 58)
        .overlay(Capsule().stroke(tint.opacity(0.16), lineWidth: 1))
        .scaleEffect(progress > 0 && !complete && !reduceMotion ? 0.97 : 1)
        .contentShape(Capsule())
        .gesture(DragGesture(minimumDistance: 0)
            .onChanged { value in
                guard !complete, !cancelled else { return }
                if abs(value.translation.width) > 28 || abs(value.translation.height) > 28 {
                    cancelled = true
                    cancelHold()
                } else if holdTask == nil {
                    beginHold()
                }
            }
            .onEnded { _ in
                cancelled = false
                if !complete { cancelHold() }
            })
        .accessibilityElement(children: .ignore)
        .accessibilityLabel(complete ? "Archived" : title)
        .accessibilityHint("Activate to confirm without holding")
        .accessibilityAddTraits(.isButton)
        .accessibilityAction { confirm() }
        .sensoryFeedback(.success, trigger: complete)
        .task(id: complete) {
            guard complete else { return }
            do { try await Task.sleep(for: .seconds(1.6)) } catch { return }
            complete = false
            progress = 0
        }
        .onDisappear {
            cancelHold()
            cancelled = false
        }
    }

    private func beginHold() {
        holdTask = Task { @MainActor in
            let clock = ContinuousClock()
            let start = clock.now
            let safeDuration = max(0.2, duration)
            while !Task.isCancelled {
                let elapsed = start.duration(to: clock.now).components
                let seconds = Double(elapsed.seconds) + Double(elapsed.attoseconds) / 1e18
                progress = min(seconds / safeDuration, 1)
                if progress >= 1 { confirm(); return }
                do { try await Task.sleep(for: .milliseconds(16)) } catch { return }
            }
        }
    }

    private func cancelHold() {
        holdTask?.cancel()
        holdTask = nil
        withAnimation(reduceMotion ? nil : .easeOut(duration: 0.18)) { progress = 0 }
    }

    private func confirm() {
        guard !complete else { return }
        holdTask?.cancel()
        holdTask = nil
        progress = 1
        complete = true
        onConfirm()
    }
}

struct HoldToConfirmDemo: View {
    @State private var archivedCount = 0
    var body: some View {
        VStack(spacing: 28) {
            Image(systemName: "tray.full").font(.system(size: 38, weight: .light))
            Text("A little more intentional.").font(.title3.weight(.semibold))
            HoldToConfirmButton { archivedCount += 1 }
            Text("Archived: \(archivedCount)").font(.footnote).foregroundStyle(.secondary)
        }.padding(32).frame(maxWidth: .infinity, maxHeight: .infinity)
    }
}

#Preview { HoldToConfirmDemo() }
