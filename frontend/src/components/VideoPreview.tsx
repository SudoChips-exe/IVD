import type { VideoInfo } from '../types';
import { Play, User, Clock } from './Icons';

interface VideoPreviewProps {
  info: VideoInfo;
}

export default function VideoPreview({ info }: VideoPreviewProps) {
  return (
    <div className="card preview-card" data-screen-label="VideoPreview">
      <div className="thumb">
        {info.thumbnail && <img src={info.thumbnail} alt={info.title} />}
        <span className="play"><Play /></span>
        {info.duration && <span className="dur">{info.duration}</span>}
      </div>

      <div className="meta">
        {info.source && <span className="src-tag">{info.source}</span>}
        <h3>{info.title}</h3>
        <div className="meta-rows">
          {info.author && (
            <div className="meta-row"><User /> {info.author}</div>
          )}
          {info.meta && (
            <div className="meta-row"><Clock /> {info.meta}</div>
          )}
        </div>
      </div>
    </div>
  );
}
