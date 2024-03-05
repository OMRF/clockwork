<script lang="ts" context="module">
    import { z } from 'zod'

    export const schema = z.object({
        allowedDays: z.array(z.number()).min(1, { message: 'Really? You\'re not available on any day?' }),
        ageMonths: z.coerce.number().int().min(0),
        toleranceDays: z.coerce.number().int().min(0),
    })
</script>

<script lang="ts">
    import { invoke } from '@tauri-apps/api'
    import AllowedDaysCard from './AllowedDaysCard.svelte'
    import RatAgeCard from './RatAgeCard.svelte'
    import { defaults, superForm } from 'sveltekit-superforms'
    import { zod } from 'sveltekit-superforms/adapters'
    import * as Form from '$lib/components/ui/form'
    import Spinner from '$lib/components/Spinner.svelte'
    import { toast } from 'svelte-sonner'

    const click = async () => {
        const response = await invoke('test')

        console.log(response)
    }

    const form = superForm(defaults({
        allowedDays: [1, 2, 3, 4, 5],
        ageMonths: 24,
        toleranceDays: 10,
    }, zod(schema)), {
        SPA: true,
        validators: zod(schema),
        validationMethod: 'oninput',
        onUpdate: async ({ form }) => {
            if (!form.valid) return

            const data = form.data

            try {
                await invoke('test', {
                    allowedDays: data.allowedDays,
                    ageMonths: data.ageMonths,
                    toleranceDays: data.toleranceDays,
                })
            } catch (e) {
                toast.error(e as string)
            }

        },
    })

    const { enhance, submitting } = form
</script>

<h2 class="font-semibold tracking-tight text-3xl">Constraints</h2>
<form method="POST" use:enhance>
    <div class="grid gap-4 grid-cols-1 md:grid-cols-2 mt-3">
        <AllowedDaysCard {form} />
        <RatAgeCard {form} />
    </div>
    <Form.Button class=" mt-3 float-right">
        {#if $submitting}
            <Spinner class="h-4 w-4 mr-2" />
        {/if}
        { " " }
        Select file & process
    </Form.Button>
</form>
