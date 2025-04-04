import { WeightsMap } from "tetris-ai";

class WeightEntry {
    enabled = $state(true);
    value = $state(0);

    constructor(enabled = true, value = 0) {
        this.enabled = enabled;
        this.value = value;
    }
}

export type WeightsObject = { [key: string]: number };

export class Weights {
    map: Map<string, WeightEntry> = $state(new Map<string, WeightEntry>());

    constructor(allowedFeatures?: string[]) {
        for (const [key, value] of WeightsMap.defaults().into_js()) {
            if (allowedFeatures && !allowedFeatures.includes(key)) {
                continue;
            }
            this.map.set(key, new WeightEntry(true, value));
        }
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
