import SwiftUI

@main
struct SwiftUIStudiesApp: App {
    var body: some Scene {
        WindowGroup {
            NavigationStack {
                if let slug = ProcessInfo.processInfo.arguments.last,
                   ProcessInfo.processInfo.arguments.contains("--demo") {
                    destination(slug).navigationTitle(slug).navigationBarTitleDisplayMode(.inline)
                } else {
                    List {
                        NavigationLink("Photo Stack") { PhotoStackDemo() }
                        NavigationLink("Hold to Confirm") { HoldToConfirmDemo() }
                        NavigationLink("Mini Player") { MiniPlayerDemo() }
                        NavigationLink("Reaction Picker") { ReactionPickerDemo() }
                        NavigationLink("Send Button") { SendButtonDemo() }
                        NavigationLink("Pull to Search") { PullToSearchDemo() }
                    }.navigationTitle("Just Do Swift")
                }
            }
        }
    }
    @ViewBuilder private func destination(_ slug: String) -> some View {
        switch slug {
        case "photo-stack": PhotoStackDemo()
        case "hold-to-confirm": HoldToConfirmDemo()
        case "mini-player": MiniPlayerDemo()
        case "reaction-picker": ReactionPickerDemo()
        case "send-button": SendButtonDemo()
        case "pull-to-search": PullToSearchDemo()
        default: Text("Unknown component")
        }
    }
}
