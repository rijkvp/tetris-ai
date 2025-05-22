<script lang="ts">
    import { locale, locales, localeNames, setLocale } from "$lib/translations";
    import { base } from "$app/paths";
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
</script>

<div>
    <select
        onchange={(e) => {
            const lang = (e.target as HTMLSelectElement).value;
            if (lang === "nl" || lang === "en") setLocale(lang);

            let appPath = page.url.pathname;
            appPath = appPath.substring(base.length); // remove base
            const path = appPath.replace(/^\/[a-zA-Z]+/, ""); // remove lang
            goto(`${base}/${lang}${path}`);
        }}
    >
        {#each $locales as lc}
            <option value={lc} selected={lc === $locale}>
                {localeNames[lc as keyof typeof localeNames]}</option
            >
        {/each}
    </select>
</div>
