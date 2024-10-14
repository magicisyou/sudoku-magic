import { writable } from "svelte/store";

export const gameLevel = writable<number>(1);
export const unlockedLevel = writable<number>(1);
