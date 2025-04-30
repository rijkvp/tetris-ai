import { WeightsMap } from "tetris-ai";

class WeightEntry {
    enabled = $state(true);
    value = $state(0);
    locked = $state(false);

    constructor(enabled = true, value = 0, locked = false) {
        this.enabled = enabled;
        this.value = value;
        this.locked = locked;
    }
}

export type WeightsObject = { [key: string]: number };

export class Weights {
    map: Map<string, WeightEntry> = $state(new Map<string, WeightEntry>());

    constructor(defaultFeatures?: [string, number][], lockedFeatures?: string[]) {
        for (const [key, value] of WeightsMap.defaults().into_js()) {
            if (defaultFeatures && !defaultFeatures.includes(key)) {
                continue;
            }
            const initialValue = defaultFeatures?.find(([k]) => k === key)?.[1] ?? value;
            const locked = lockedFeatures?.includes(key) ?? false;
            this.map.set(key, new WeightEntry(true, initialValue, locked));
        }
    }

    static fromValues(keys: string[], values: number[]) {
        const weights = new Weights();
        for (let i = 0; i < keys.length; i++) {
            weights.map.set(keys[i], new WeightEntry(true, values[i]));
        }
        return weights;
    }

    getWeightsMap(): WeightsMap {
        const array = new Array();
        for (const [key, entry] of this.map) {
            if (entry.enabled) {
                array.push([key, entry.value]);
            }
        }
        return WeightsMap.from_js(array);
    }

    getObject(): WeightsObject {
        let object: WeightsObject = {};
        for (const [key, entry] of this.map) {
            object[key] = entry.value;
        }
        return object;
    }

    entries() {
        return this.map.entries();
    }

    set(key: string, entry: WeightEntry) {
        this.map.get(key)!.enabled = entry.enabled;
        this.map.get(key)!.value = entry.value;
    }

    #setValue(key: string, value: number) {
        if (this.map.has(key)) {
            this.map.get(key)!.enabled = true;
            this.map.get(key)!.value = value;
        }
    }

    setWeights(other: WeightsObject) {
        for (const [key, value] of Object.entries(other)) {
            this.#setValue(key, value);
        }
    }

    get(key: string) {
        return this.map.get(key);
    }

    reset() {
        for (const entry of this.map.values()) {
            entry.enabled = true;
            entry.value = 0;
        }
    }

    randomize() {
        for (const entry of this.map.values()) {
            if (entry.enabled) {
                entry.value = parseFloat(
                    (Math.random() * 20 - 10).toFixed(1),
                );
            }
        }
    }

    loadPreset(name: string) {
        const preset = WeightsMap.preset(name).into_js();
        if (preset) {
            for (const [key, value] of preset) {
                this.#setValue(key, value);
            }
        }
    }
}
