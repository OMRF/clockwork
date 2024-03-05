<script lang="ts" context="module">
    export const daysOfWeek = [
        { id: 0, name: 'Sunday' },
        { id: 1, name: 'Monday' },
        { id: 2, name: 'Tuesday' },
        { id: 3, name: 'Wednesday' },
        { id: 4, name: 'Thursday' },
        { id: 5, name: 'Friday' },
        { id: 6, name: 'Saturday' },
    ]
</script>

<script lang="ts">
    import { Checkbox } from '$lib/components/ui/checkbox'
    import { Label } from '$lib/components/ui/label'
    import * as Card from '$lib/components/ui/card'
    import type { Infer, SuperForm } from 'sveltekit-superforms'
    import { schema } from './+page.svelte'
    import * as Form from '$lib/components/ui/form'

    export let form: SuperForm<Infer<typeof schema>>

    const { form: formData } = form
    const add = (id: number) => {
        $formData.allowedDays = [...$formData.allowedDays, id]
    }

    const remove = (id: number) => {
        $formData.allowedDays = $formData.allowedDays.filter((day) => day !== id)
    }
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Allowed Days of Week</Card.Title>
    </Card.Header>
    <Card.Content>
        <Form.Fieldset {form} name="allowedDays" class="space-y-2">
            {#each daysOfWeek as day}
                {@const checked = $formData.allowedDays.includes(day.id)}
                <div class="flex items-center space-x-2">
                    <Form.Control let:attrs>
                        <Checkbox
                            {...attrs}
                            {checked}
                            onCheckedChange={
                                (v) => {
                                    if (v) {
                                        add(day.id)
                                    } else {
                                        remove(day.id)
                                    }
                            }} />
                        <Form.Label class="font-normal">{day.name}</Form.Label>
                        <input hidden type="checkbox" name={attrs.name} value={day.id.toString()} {checked} />
                    </Form.Control>
                </div>
            {/each}
            <Form.FieldErrors />
        </Form.Fieldset>
    </Card.Content>
</Card.Root>