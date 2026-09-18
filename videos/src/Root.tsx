import "./index.css";
import { HoldComposition } from "./HoldToConfirm";
import { PhotoComposition } from "./PhotoStack";
import { PlayerComposition } from "./MiniPlayer";
import { ReactionComposition } from "./ReactionPicker";
import { SendComposition } from "./SendButton";
import { SearchComposition } from "./PullToSearch";

export const RemotionRoot: React.FC = () => {
  return (
    <>
      <PhotoComposition />
      <HoldComposition />
      <PlayerComposition />
      <ReactionComposition />
      <SendComposition />
      <SearchComposition />
    </>
  );
};
