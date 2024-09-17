import Sylvie from 'sylviejs';
import { appDataDir, join } from '@tauri-apps/api/path';
import { exists, readTextFile, remove, writeTextFile } from '@tauri-apps/plugin-fs';
import type { IncrementalPersistenceAdapter } from 'sylviejs/storage-adapter/src/models/persistence-adapter';
import type { PersistenceAdapterCallback } from 'sylviejs/storage-adapter/src/models/persistence-adapter-callback';

let db: Sylvie;
let dbPath: string;

class PersistenceAdapter implements IncrementalPersistenceAdapter {
  mode: 'incremental' = 'incremental';

  async loadDatabase(dbname: string): Promise<string> {
    console.log("load");
    if (await exists(dbname)) {
      return await readTextFile(dbname);
    }
    return '';
  }

  async saveDatabase(dbname: string, dbref: () => Sylvie, callback?: PersistenceAdapterCallback): Promise<void> {
    console.log("save");
    const dbstring = JSON.stringify(dbref());
    await writeTextFile(dbname, dbstring);
    if (callback) callback(undefined);
  }

  async deleteDatabase(dbname: string): Promise<void> {
    if (await exists(dbname)) {
      await remove(dbname);
    }
  }
}

export async function getDb(): Promise<Sylvie> {
  if (!db) {
    const appDataDirPath = await appDataDir();
    dbPath = await join(appDataDirPath, 'data.db');

    const adapter = new PersistenceAdapter();

    const initialData = await adapter.loadDatabase(dbPath);

    db = new Sylvie(dbPath, {
      adapter: adapter,
      autosave: true,
      autosaveInterval: 1000,
      autoload: false
    });

    if (initialData) {
      db.loadJSON(initialData);
    }

    const collectionNames = ['history', 'playlists', 'settings', 'songs'];
    collectionNames.forEach(collectionName => {
      if (!db.getCollection(collectionName)) {
        db.addCollection(collectionName);
      }
    });

    if (collectionNames.some(name => !db.getCollection(name))) {
      await saveDb();
    }
  }
  return db;
}

export async function saveDb(): Promise<void> {
  if (db && dbPath) {
    db.saveDatabase();
  }
}