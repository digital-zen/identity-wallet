<script lang="ts">
  import type { HistoryEvent } from '@bindings/HistoryEvent';
  import ListItemCard from '$lib/components/molecules/ListItemCard.svelte';

  import { EventType } from '.';

  // export let connection: Connection;
  export let event: HistoryEvent = {
    image_id: 'none',
    date: '0000-00-00',
    title: 'Nothing',
    sub_title: 'Nothing',
    event_type: EventType.INITIAL_CONNECTION,
  };
</script>

<div class="flex flex-col">
  <p class="text-[14px]/[22px] font-medium text-slate-800">{event.title}</p>
  <p class="text-[12px]/[20px] font-medium text-slate-500">
    {new Date(event.date).toLocaleString('en-US', {
      dateStyle: 'medium',
      timeStyle: 'medium',
    })}
  </p>
    <div class="mt-[12px] rounded-xl border border-slate-200 bg-white p-[3px] dark:border-slate-600">
        <ListItemCard
          id="some_id_todo"
          title={event.title}
          description={event.sub_title}
        >
          <!-- <span slot="icon"><EnvelopeSimple class="h-6 w-6" /></span> -->
        </ListItemCard>
    </div>
</div>
