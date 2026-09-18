import SwiftUI

@MainActor
struct SendButton: View {
    var enabled = true
    let send: @MainActor () async throws -> Void
    @State private var state: SendState = .idle
    @State private var task: Task<Void, Never>?
    @Environment(\.accessibilityReduceMotion) private var reduceMotion

    private enum SendState { case idle, sending, sent, failed }
    private var title: String {
        switch state {
        case .idle: "Send message"
        case .sending: "Sending"
        case .sent: "Sent"
        case .failed: "Try again"
        }
    }
    private var symbol: String {
        switch state {
        case .idle: "arrow.up"
        case .sending: "arrow.triangle.2.circlepath"
        case .sent: "checkmark"
        case .failed: "arrow.clockwise"
        }
    }
    var body: some View {
        VStack(spacing: 10) {
            Button(action: submit) {
                HStack(spacing: 10) {
                    if state == .sending { ProgressView().tint(.white) }
                    else { Image(systemName: symbol).contentTransition(.symbolEffect(.replace)) }
                    Text(title).contentTransition(.opacity)
                }
                .font(.system(.subheadline, design: .rounded, weight: .semibold))
                .foregroundStyle(.white).frame(width: state == .sent ? 122 : 182, height: 52)
                .background(state == .sent ? Color(red: 0.25, green: 0.45, blue: 0.37) : .black, in: Capsule())
                .opacity(enabled ? 1 : 0.4)
            }
            .buttonStyle(.plain)
            .disabled(!enabled || state == .sending || state == .sent)
            .accessibilityLabel(title)
            .animation(reduceMotion ? nil : .spring(response: 0.35, dampingFraction: 0.8), value: state)
            if state == .failed { Text("Couldn’t send. Your message is still here.").font(.caption).foregroundStyle(.secondary) }
        }
        .sensoryFeedback(.success, trigger: state == .sent)
        .onDisappear { task?.cancel() }
    }
    private func submit() {
        guard enabled, state == .idle || state == .failed else { return }
        state = .sending
        task = Task { @MainActor in
            do {
                try await send()
                try Task.checkCancellation()
                state = .sent
                try await Task.sleep(for: .seconds(1.5))
                state = .idle
            } catch is CancellationError { state = .idle }
            catch { state = .failed }
        }
    }
}

struct SendButtonDemo: View {
    @State private var message = "See you at golden hour."
    @State private var simulateFailure = false
    private enum DemoError: Error { case offline }
    var body: some View {
        VStack(alignment: .leading, spacing: 22) {
            Text("A small hello.").font(.title2.weight(.semibold))
            TextField("Write a message…", text: $message, axis: .vertical)
                .lineLimit(3...5).padding(20)
                .background(Color(.secondarySystemGroupedBackground), in: RoundedRectangle(cornerRadius: 18))
            HStack {
                Spacer()
                SendButton(enabled: !message.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty) {
                    // Replace this delay with your real async request.
                    try await Task.sleep(for: .seconds(1.2))
                    if simulateFailure { throw DemoError.offline }
                }
            }
            Toggle("Simulate a failed request", isOn: $simulateFailure).font(.caption)
        }.padding(28).frame(maxWidth: 380)
            .frame(maxWidth: .infinity, maxHeight: .infinity)
            .background(Color(.systemGroupedBackground))
    }
}

#Preview { SendButtonDemo() }
