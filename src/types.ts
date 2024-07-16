export type AudioMetadata = {
  title: string;
  asin: string;
  exportTime: string;
  records: Record[];
};

export type Record = {
  Type: string;
  Created: string;
  Start: string;
  AnnotationId?: string;
  LastModified?: string;
  RecordType: string;
  End?: string;
  Text: any;
  Title: any;
  relativePosition: number;
  hidden: boolean;
};
