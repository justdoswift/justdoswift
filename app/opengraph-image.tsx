import { ImageResponse } from "next/og";

export const alt = "Just Do Swift — Beautiful SwiftUI components";
export const size = { width: 1200, height: 630 };
export const contentType = "image/png";

export default function OpenGraphImage() {
  return new ImageResponse(
    (
      <div
        style={{
          width: "100%",
          height: "100%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
          padding: "62px 72px",
          color: "#f5f5f2",
          background:
            "radial-gradient(circle at 78% 28%, rgba(105,130,235,.3), transparent 34%), linear-gradient(145deg, #09090b, #111116)",
          fontFamily: "Arial, sans-serif",
        }}
      >
        <div style={{ display: "flex", alignItems: "center", gap: 18, fontSize: 24, fontWeight: 700 }}>
          <div
            style={{
              width: 48,
              height: 48,
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              border: "1px solid rgba(255,255,255,.2)",
              borderRadius: 15,
              background: "#19191e",
            }}
          >
            <div style={{ width: 19, height: 25, borderRadius: 8, background: "linear-gradient(145deg,#fff,#898a93)", transform: "rotate(40deg)" }} />
          </div>
          Just Do Swift
        </div>
        <div style={{ display: "flex", flexDirection: "column" }}>
          <div style={{ color: "#91a8ff", fontSize: 17, letterSpacing: 5, textTransform: "uppercase", marginBottom: 24 }}>
            Curated SwiftUI library
          </div>
          <div style={{ display: "flex", flexDirection: "column", fontSize: 82, lineHeight: .96, letterSpacing: -5, fontWeight: 700 }}>
            <span>Make SwiftUI</span>
            <span style={{ color: "#8f8f99" }}>feel alive.</span>
          </div>
        </div>
        <div style={{ display: "flex", justifyContent: "space-between", color: "#8b8b94", fontSize: 18 }}>
          <span>Preview the motion. Own the source.</span>
          <span>justdoswift.com</span>
        </div>
      </div>
    ),
    size,
  );
}
