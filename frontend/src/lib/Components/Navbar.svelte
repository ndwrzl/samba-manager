<script lang="ts">
	import { logout } from '$lib/api';
</script>

<nav>
	<p>SAMBA</p>
	<button on:click={logout}> Logout </button>
</nav>

<style lang="scss">
	nav {
		@apply flex items-center justify-between w-full p-2;
		background-color: var(--secondary);
		p {
			@apply h-min;
		}
		button {
			@apply mr-3 inline-block border border-blue-500 rounded py-2 px-4 bg-blue-500 hover:bg-blue-700 text-white;
		}
	}
</style>
