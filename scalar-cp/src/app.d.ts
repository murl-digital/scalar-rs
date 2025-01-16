// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      email?: string;
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }

  declare type FileDropEvent = import("filedrop-svelte/event").FileDropEvent;
  declare type FileDropSelectEvent =
    import("filedrop-svelte/event").FileDropSelectEvent;
  declare type FileDropDragEvent =
    import("filedrop-svelte/event").FileDropDragEvent;
  declare namespace svelteHTML {
    interface HTMLAttributes<T> {
      onfiledrop?: (
        event: CustomEvent<FileDropSelectEvent> & { target: EventTarget & T },
      ) => void;
      onfiledragenter?: (
        event: CustomEvent<FileDropDragEvent> & { target: EventTarget & T },
      ) => void;
      onfiledragleave?: (
        event: CustomEvent<FileDropDragEvent> & { target: EventTarget & T },
      ) => void;
      onfiledragover?: (
        event: CustomEvent<FileDropDragEvent> & { target: EventTarget & T },
      ) => void;
      onfiledialogcancel?: (
        event: CustomEvent<FileDropEvent> & { target: EventTarget & T },
      ) => void;
      onfiledialogclose?: (
        event: CustomEvent<FileDropEvent> & { target: EventTarget & T },
      ) => void;
      onfiledialogopen?: (
        event: CustomEvent<FileDropEvent> & { target: EventTarget & T },
      ) => void;
      onwindowfiledragenter?: (
        event: CustomEvent<FileDropDragEvent> & { target: EventTarget & T },
      ) => void;
      onwindowfiledragleave?: (
        event: CustomEvent<FileDropDragEvent> & { target: EventTarget & T },
      ) => void;
      onwindowfiledragover?: (
        event: CustomEvent<FileDropDragEvent> & { target: EventTarget & T },
      ) => void;
    }
  }
}

export {};
