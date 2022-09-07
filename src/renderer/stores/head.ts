import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { ProjectHead } from "../typings/proteus";

export const useHeadStore = defineStore("head", () => {
  /////////////
  //  STORE  //
  /////////////

  const head = ref({ name: "untitled", path: "" } as ProjectHead);

  /////////////
  // GETTERS //
  /////////////

  const name = computed(() => head.value.name)
  const path = computed(() => head.value.path)

  /////////////
  // SETTERS //
  /////////////

  const setFileLocation = (location: string) => {
    head.value.name = (location.match(/[^\/\\]*\.\w+$/) || [".jpg"])[0].replace(/\.\w+$/, "");
    head.value.path = location;
  };

  const setName = (name:string) => {head.value.name = name}
  const setPath = (location:string) => {head.value.path = location}

  return {
    name,
    path, 
    setFileLocation,
    setName,
    setPath
  };
});
