<script lang="ts">
    import { dispatch } from '$lib/dispatcher';
    import { state } from '$src/stores';
    import Ascending from '~icons/ph/sort-ascending';
    import Issued from '~icons/ph/calendar-check';
    import Added from '~icons/ph/calendar-plus';
  </script>
  
  <!-- To do: add dark version, remove extra li items, checkmarks, close bottomdrawer-->
  <div class="w-full rounded-t-xl bg-white p-5">
    <div class="m-2 flex items-center justify-center gap-2">
      <h3 class="font-medium">Sorting</h3>
    </div>
  
    <div>
      <!--
   <ul class="relative">
        <li class=" flex cursor-pointer gap-2 rounded-lg p-3 hover:bg-slate-200">
          <List />
          <p class="text-sm font-medium">List view</p>
        </li>
  
        <li class=" flex cursor-pointer gap-2 rounded-lg p-3 hover:bg-slate-200">
          <Grid />
          <p class="text-sm font-medium">Grid view</p>
        </li>
      </ul>
      -->
  
      <ul class="relative">
        <li class="flex cursor-pointer gap-2 rounded-lg p-3 hover:bg-stone-50">
          <Ascending />
          <p class="text-sm font-medium">Alphabetical</p>
        </li>
  
        <li class=" flex cursor-pointer gap-2 rounded-lg p-3 hover:bg-stone-50">
          <Issued />
          <p class="text-sm font-medium">Date issued</p>
        </li>
  
        <li class=" flex cursor-pointer gap-2 rounded-lg p-3 hover:bg-stone-50">
          <Added />
          <p class="text-sm font-medium">Date added</p>
        </li>
      </ul>
  
      <button class="my-2 h-12 w-full items-center justify-center rounded-xl bg-primary font-medium text-white"
        >Continue
      </button>
    </div>
  </div>