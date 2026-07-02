import type { DownloadJob, DownloadStatus } from '../types';
import { Close, Trash, External } from './Icons';

const STATUS_LABEL: Record<DownloadStatus, string> = {
  queued: 'Queued',
  downloading: 'Downloading',
  completed: 'Completed',
  error: 'Failed',
};

interface DownloadQueueItemProps {
  job: DownloadJob;
  onCancel: (id: string) => void;
  onRemove: (id: string) => void;
}

export default function DownloadQueueItem({ job, onCancel, onRemove }: DownloadQueueItemProps) {
  const active = job.status === 'downloading' || job.status === 'queued';

  return (
    <div className="card q-item">
      <div className="q-thumb">
        {job.thumbnail && <img src={job.thumbnail} alt="" />}
      </div>

      <div className="q-body">
        <div className="q-top">
          <div className="q-title">{job.title}</div>
          <div className="q-actions">
            {job.status === 'completed' && (
              <button className="icon-btn" type="button" title="Open file">
                <External />
              </button>
            )}
            {active ? (
              <button
                className="icon-btn danger"
                type="button"
                title="Cancel"
                onClick={() => onCancel(job.id)}
              >
                <Close />
              </button>
            ) : (
              <button
                className="icon-btn"
                type="button"
                title="Remove"
                onClick={() => onRemove(job.id)}
              >
                <Trash />
              </button>
            )}
          </div>
        </div>

        <div className="progress">
          <i style={{ width: `${Math.max(0, Math.min(100, job.progress))}%` }} />
        </div>

        <div className="q-foot">
          <span className={`badge ${job.status}`}>
            <span className="bdot" />
            {STATUS_LABEL[job.status]}
          </span>
          <span>{job.detail}</span>
        </div>
      </div>
    </div>
  );
}
