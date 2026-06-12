// VIDCLAW icon set. lightweight inline SVGs, stroke = currentColor.
import type { SVGProps } from 'react';

type Icon = (props: SVGProps<SVGSVGElement>) => JSX.Element;

const base: SVGProps<SVGSVGElement> = {
  viewBox: '0 0 24 24',
  fill: 'none',
  xmlns: 'http://www.w3.org/2000/svg',
};

export const ClawMark: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M12 3v11m0 0 4.5-4.5M12 14 7.5 9.5" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M5 17.5v1A2.5 2.5 0 0 0 7.5 21h9a2.5 2.5 0 0 0 2.5-2.5v-1" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" />
  </svg>
);

export const Moon: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M21 12.8A9 9 0 1 1 11.2 3a7 7 0 0 0 9.8 9.8Z" stroke="currentColor" strokeWidth="1.8" strokeLinejoin="round" />
  </svg>
);

export const Sun: Icon = (p) => (
  <svg {...base} {...p}>
    <circle cx="12" cy="12" r="4.2" stroke="currentColor" strokeWidth="1.8" />
    <path d="M12 2v2M12 20v2M2 12h2M20 12h2M5 5l1.5 1.5M17.5 17.5 19 19M19 5l-1.5 1.5M6.5 17.5 5 19" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" />
  </svg>
);

export const Link: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M10 14a3.5 3.5 0 0 0 5 0l3-3a3.5 3.5 0 0 0-5-5l-1 1" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M14 10a3.5 3.5 0 0 0-5 0l-3 3a3.5 3.5 0 0 0 5 5l1-1" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
);

export const Clipboard: Icon = (p) => (
  <svg {...base} {...p}>
    <rect x="8" y="3" width="8" height="4" rx="1.4" stroke="currentColor" strokeWidth="1.8" />
    <path d="M8 5H6a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" />
  </svg>
);

export const Play: Icon = (p) => (
  <svg {...base} {...p} fill="currentColor">
    <path d="M8 5v14l11-7z" />
  </svg>
);

export const User: Icon = (p) => (
  <svg {...base} {...p}>
    <circle cx="12" cy="8" r="3.5" stroke="currentColor" strokeWidth="1.8" />
    <path d="M5 20a7 7 0 0 1 14 0" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" />
  </svg>
);

export const Clock: Icon = (p) => (
  <svg {...base} {...p}>
    <circle cx="12" cy="12" r="9" stroke="currentColor" strokeWidth="1.8" />
    <path d="M12 7v5l3 2" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" />
  </svg>
);

export const Check: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="m5 13 4 4L19 7" stroke="currentColor" strokeWidth="2.4" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
);

export const DownloadCloud: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M12 3v12m0 0 4.5-4.5M12 15l-4.5-4.5" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M4 17v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" />
  </svg>
);

export const Close: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M6 6l12 12M18 6 6 18" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
  </svg>
);

export const Trash: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M5 7h14M10 7V5h4v2m-7 0 1 12h8l1-12" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
);

export const External: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M7 17 17 7M9 7h8v8" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
);

export const Cookie: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M12 3a9 9 0 1 0 9 9 3 3 0 0 1-3-3 3 3 0 0 1-3-3 3 3 0 0 1-3-3Z" stroke="currentColor" strokeWidth="1.8" strokeLinejoin="round" />
    <circle cx="9" cy="13" r="1" fill="currentColor" />
    <circle cx="13" cy="16" r="1" fill="currentColor" />
    <circle cx="15" cy="11" r="1" fill="currentColor" />
  </svg>
);

export const Chevron: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="m6 9 6 6 6-6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
  </svg>
);

export const Upload: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M12 16V4m0 0 4 4m-4-4-4 4" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M4 16v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" />
  </svg>
);

export const Inbox: Icon = (p) => (
  <svg {...base} {...p}>
    <path d="M3 13h4l2 3h6l2-3h4" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round" />
    <path d="M5 7l-2 6v5a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5l-2-6a2 2 0 0 0-2-1.4H7A2 2 0 0 0 5 7Z" stroke="currentColor" strokeWidth="1.8" strokeLinejoin="round" />
  </svg>
);
