<script lang="ts">
    import { t, locale, locales, setLocale } from "$lib/translations";
    import { base } from "$app/paths";
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
</script>

<select
    onchange={(e) => {
        const lang = (e.target as HTMLSelectElement).value;
        setLocale(lang);
        const path = page.url.pathname.replace(/^\/[a-zA-Z]+/, "");
        goto(`${base}/${lang}${path}`);
    }}
>
    {#each $locales as lc}
        <option value={lc} selected={lc === $locale}
            >{$t(`locale.${lc}`)}</option
        >
    {/each}
</select>
