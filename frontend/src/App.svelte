<script>
  import Guest from './components/Guest.svelte'
  import {Route, router} from 'tinro'

</script>

<main>
  <Route path="/">
    <div class="fancy">
      song voter
    </div>
  </Route>

  <Route path="/guest">
  </Route>

</main>

<style>
.fancy {
  text-shadow: 0 0 80px rgba(236, 255, 192, 0.75), 0 0 32px rgba(189, 255, 65, 0.24); 
  background: linear-gradient(to right, #d4ff00, #ffdf00);
  background-clip: text;
  -webkit-text-fill-color: transparent;
}

div {
  width: 1; 
  margin: auto;
  text-align: center;
  font-size: 8rem;
}
</style>
