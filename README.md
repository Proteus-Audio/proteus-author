Proteus (.prot) file authoring **Vue3** + **Electron** TypeScript based application based on [Deluze's electron-vue-template](https://github.com/Deluze/electron-vue-template). This repo is moving from the same idea as the [multiplay mixer](https://github.com/howardah/multiplay_mixer) flutter application.

### Install dependencies

```bash
yarn
```

### Start developing ⚒️

```bash
yarn dev
```

## Additional Commands

```bash
yarn dev # starts application with hot reload
yarn build # builds application

# OR

yarn build:win # uses windows as build target
yarn build:mac # uses mac as build target
yarn build:linux # uses linux as build target
```

## Using static files

If you have any files that you want to copy over to the app directory after installation, you will need to add those files in your `src/main/static` directory.

#### Referencing static files from your main process

```ts
/* Assumes src/main/static/myFile.txt exists */

import {app} from 'electron';
import {join} from 'path';
import {readFileSync} from 'fs';

const path = join(app.getAppPath(), 'static', 'myFile.txt');
const buffer = readFileSync(path);
```
