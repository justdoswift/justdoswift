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
          color: "#161616",
          background: "#fdfdfd",
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
              borderRadius: 15,
              background: "#161616",
            }}
          >
            <span style={{ color: "#fff", fontSize: 26 }}>J</span>
          </div>
          Just Do Swift.
        </div>
        <div style={{ display: "flex", flexDirection: "column" }}>
          <div style={{ color: "#e77749", fontSize: 18, marginBottom: 24 }}>
            Built with SwiftUI
          </div>
          <div style={{ display: "flex", flexDirection: "column", fontSize: 76, lineHeight: 1.08, letterSpacing: -4, fontWeight: 700 }}>
            <span>Small components.</span>
            <span style={{ color: "#777" }}>Delightful interactions.</span>
          </div>
        </div>
        <div style={{ display: "flex", justifyContent: "space-between", color: "#727272", fontSize: 18 }}>
          <span>Preview the motion. Own the source.</span>
          <span>justdoswift.com</span>
        </div>
      </div>
    ),
    size,
  );
}
