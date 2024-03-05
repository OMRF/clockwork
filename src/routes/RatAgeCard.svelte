<script lang="ts">
    import { Checkbox } from '$lib/components/ui/checkbox'
    import { Label } from '$lib/components/ui/label'
    import * as Card from '$lib/components/ui/card'
    import { defaults, type Infer, type SuperForm, superForm, type SuperValidated } from 'sveltekit-superforms'
    import { zod } from 'sveltekit-superforms/adapters'
    import { z } from 'zod'
    import * as Form from '$lib/components/ui/form'
    import { Input } from '$lib/components/ui/input'
    import { schema } from './+page.svelte'


    export let form: SuperForm<Infer<typeof schema>>

    const { form: formData } = form
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Rodent Age</Card.Title>
    </Card.Header>
    <Card.Content class="flex flex-col space-y-2">
        <Form.Field {form} name="ageMonths">
            <Form.Control let:attrs>
                <Form.Label>Target Age (months)</Form.Label>
                <Input {...attrs} bind:value={$formData.ageMonths} />
            </Form.Control>
            <Form.FieldErrors />
        </Form.Field>

        <Form.Field {form} name="toleranceDays">
            <Form.Control let:attrs>
                <Form.Label>Tolerance (±days)</Form.Label>
                <Input {...attrs} bind:value={$formData.toleranceDays} />
            </Form.Control>
            <Form.Description>Acceptable range within the target</Form.Description>
            <Form.FieldErrors />
        </Form.Field>
    </Card.Content>
</Card.Root>