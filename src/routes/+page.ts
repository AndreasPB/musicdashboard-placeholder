import type { PageLoad } from './$types';
import { invoke } from "@tauri-apps/api/tauri";

export const load: PageLoad = async () => {
    const hest: string = await invoke("hello")
    const genres: string = JSON.parse(await invoke("get_genres"))

    console.log(genres);
    

    return {hest, genres};
};