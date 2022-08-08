<script>
  import Guest from './components/Guest.svelte'
  import {Route, router} from 'tinro'

</script>

<main>
  <Route path="/">
    <div class="fg-light">
      Song
    </div>
    <div class="fg-dark">
      Voter
    </div>
  </Route>

  <Route path="/guest">
  </Route>

</main>

<style>
</style>
