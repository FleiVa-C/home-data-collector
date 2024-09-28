<script>
	import { ModeWatcher } from 'mode-watcher';
	import '../app.css';
</script>


<div class="app">

	<main>
		<ModeWatcher />
		<slot></slot>
	</main>

	<footer>
	</footer>
</div>
