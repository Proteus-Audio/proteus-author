<template>
  <div v-bind="getRootProps()" :class="`track-bin ${isDragActive ? 'drag' : ''}`">
    <div v-if="!fresh" class="bin">
      <!-- <input type="text" class="bin-name" placeholder="Click to Add Name" v-model="binName" /> -->
      <div type="text" class="bin-name">{{ binName }}</div>
      <span class="flex">
        <div class="waveforms">
          <TrackWaveform
            v-for="file in track.files"
            :class="`waveform ${file.id === track.selection ? 'visible' : 'hidden'}`"
            :key="file.id"
            :track="file"
            :selected="file.id === track.selection"
            >{{ file.name }}</TrackWaveform
          >
        </div>

        <el-button
          :icon="Folder"
          class="folder-button"
          @click="() => (folderOpen = !folderOpen)"
          text
        />
      </span>
      <el-drawer
        ref="folderContents"
        v-model="folderOpen"
        :title="`Track Bin Contents`"
        custom-class="drawer"
      >
        <div class="tracklist">
          <div v-for="file in track.files">
            {{ file.name }}

            <el-button :icon="Delete" class="closeButton" @click="() => removeFile(file.id)" text />
          </div>
        </div>
      </el-drawer>
    </div>

    <input v-bind="getInputProps()" />

    <span v-if="fresh" class="message clickable" @click="open">
      <p v-if="isDragActive">Drop the files here ...</p>
      <p v-else>
        Drag 'n' drop some files here, or click to select files
        <span class="error">{{ error }}</span>
      </p>
    </span>
    <p class="error" v-else>{{ error }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed, defineEmits, onUpdated, ref } from "vue";

import { useDropzone } from "vue3-dropzone";
import { useProteusStore } from "../../stores/proteus";
import TrackWaveform from "./TrackWaveform.vue";

import { Folder, Delete } from "@element-plus/icons-vue";
// import Button from "element-plus";

interface Props {
  trackId: number;
}

const props = defineProps<Props>();

const prot = useProteusStore();

const track = prot.getOrCreateTrackFromId(props.trackId);

const folderOpen = ref(false);
const error = ref("");

const errorMessage = (code: string): string => {
  type Lookup = { [key: string]: string };
  const messages: Lookup = {
    "file-invalid-type": "Please Choose a WAV or MP3 File",
  };
  if (messages[code]) return messages[code];
  return "File Error";
};

async function onDrop(acceptFiles: File[], rejectReasons: any) {
  if (rejectReasons.length > 0) error.value = errorMessage(rejectReasons[0].errors[0].code);
  else error.value = "";

  if (acceptFiles.length > 0) {
    prot.addFileToTrack(acceptFiles, props.trackId);
    prot.setTrackSelection(props.trackId);
    prot.addEmptyTrackIfNone();
  }
}

const removeFile = (id: number) => prot.removeFileFromTrack(id, props.trackId);

const binName = computed(() => {
  const filename: string | undefined = prot.getTrackSelection(props.trackId)?.name;
  return filename ? filename.replace(/\..*$/, "") : "";
})

const fresh = computed(() => {
  const isFresh = track.files.length === 0;
  return isFresh;
});

const { getRootProps, getInputProps, isDragActive, open, ...rest } = useDropzone({
  onDrop,
  accept: ["audio/mpeg", "audio/wav"],
  noClick: true,
});
</script>

<style lang="scss" scoped>
.track-bin {
  background: rgba(0, 0, 0, 0.1);
  padding: 1em;
  margin-bottom: 1em;
  border-radius: 0.5em;
  &.drag {
    background: rgba(0, 0, 0, 0.2);
  }

  .clickable {
    cursor: pointer;
  }
  .error {
    color: rgb(189, 50, 50);
  }

  .bin {
    .bin-name {
      font-family: inherit;
      background: transparent;
      border: none;
      max-width: 100%;
      width: fit-content;
      margin-bottom: 1em;

      &:focus-visible {
        border: none;
      }
    }

    .flex {
      display: flex;
      // grid-template-columns: 1fr 44px;
    }

    .waveforms {
      display: inline-block;
      width: 100%;

      .waveform {
        &.hidden {
          display: none;
        }
      }
    }

    .folder-button {
      margin-top: auto;
    }

    &:deep(.drawer) {
      min-width: 300px;

      .tracklist {
        display: grid;
        //   grid-template-columns: 1fr 45px;
      }

      //   .closeButton {
      //     display: inline-block;
      //   }
    }
  }
}
</style>
