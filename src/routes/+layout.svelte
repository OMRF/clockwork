<script>
    import ContentContainer from '$lib/components/ContentContainer.svelte'
    import Spinner from '$lib/components/Spinner.svelte'
    import { getVersion } from '@tauri-apps/api/app'
    import '$lib/assets/css/cal-sans.css'
    import '@fontsource-variable/inter'

    import '../app.pcss'
    import { Toaster } from '$lib/components/ui/sonner'
</script>

<Toaster richColors />

<div class="h-2 w-full bg-gradient-to-r from-rose-600 to-pink-200" />

<ContentContainer>
    <div class="space-y-1.5 pb-6 pt-8 lg:pt-12">
        <h1 class="font-heading text-2xl font-semibold md:text-5xl lg:text-7xl">
            Clockwork {#await getVersion()}
            <Spinner class="inline h-4 w-4" />
        {:then version}
            <span class="text-sm font-normal text-slate-500">v{version}</span>
        {/await}
        </h1>
        <p class="max-w-2xl text-muted-foreground">
            Experiment scheduler for computing rat sacrifice dates. Created by <a
            class="font-medium underline underline-offset-4"
            href="https://github.com/ericwang401"
            target="_blank"
        >
            Eric Wang
        </a>
        </p>
    </div>

    <slot />
</ContentContainer>