import { existsSync, mkdirSync, writeFile } from "fs";
import { readJson } from "fs-extra";
import { copyFile } from "node:fs/promises";
import path from "path";
import {LocalFileData} from "get-file-object-from-local-path";

interface TrackSkeleton {
  id: number;
  name: string;
  files: {
    id: number;
    path: string;
    name: string;
  }[];
}

interface Project {
  location?: string;
  tracks: TrackSkeleton[];
}

const copyFilesMakeDirs = async (src, dest): Promise<void> => {
  const destArr = dest.split(path.sep);
  let dirString = "";
  for (let i = 0; i < destArr.length - 1; i++) {
    const dir = destArr[i];
    dirString += path.sep + dir;
    if (!existsSync(dirString)) {
      mkdirSync(dirString);
    }
  }

  try {
    await copyFile(src, dest);
  } catch (error) {
    console.log("ERROR COPYING FILE:", error)
  }

};

const mkdirIfNone = (dir): void => {
    const destArr = dir.split(path.sep);
    let dirString = "";
    for (let i = 0; i < destArr.length; i++) {
      const dir = destArr[i];
      dirString += path.sep + dir;
      if (!existsSync(dirString)) {
        mkdirSync(dirString);
      }
    }
}

const save = async (tracks: TrackSkeleton[], fileLocation: string): Promise<Project> => {
  const exitTracks: TrackSkeleton[] = [];
  const promises: Promise<void>[] = [];
  const trackDir = fileLocation.replace(/\.\w+$/i, "");
//   mkdirIfNone(trackDir);
  if (tracks && tracks.length > 0) {
    console.log(trackDir);
    tracks.forEach((t) => {
      const track: TrackSkeleton = { id: t.id, name: t.name, files: [] };
      t.files.forEach((file) => {
        const filePath = `${trackDir}/track${track.id}/${file.name}`;
        promises.push(copyFilesMakeDirs(file.path, filePath));
        track.files.push({ id: file.id, name: file.name, path: filePath });
      });
      exitTracks.push(track);
    });
  }

  await Promise.all(promises);

  writeFile(fileLocation, JSON.stringify(exitTracks), () => {
    console.log("created");
  });

  return { location: fileLocation, tracks: exitTracks };
};

const load = async (fileLocation: string): Promise<TrackSkeleton[] | false> => {
  try {
    const details: TrackSkeleton[] = await readJson(fileLocation);
    // details.forEach(skeleton => {
    //     skeleton.files.forEach(f => {
    //         const fileData = new LocalFileData(f.path);

    //     })
    // })
    return details;
  } catch (err) {
    console.error(err);
    return false;
  }
};

export { save, load, Project };
