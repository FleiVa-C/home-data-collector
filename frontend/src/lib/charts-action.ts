import { Charts } from 'chart.js';

export default (node: HTMLCanvasElement, config: Chart.ChartConfiguration ) => {
	const redraw = true;
	const oneToOne = true;
	const chart = Charts.chart(node, config);

	const resizeObserver = new ResizeObserver(() => {
		chart.reflow();
	});

	resizeObserver.observe(node);

	return {
		update(config: Chart.ChartOptions) {
			chart.update(config, redraw, oneToOne);
		},
		destroy() {
			resizeObserver.disconnect();
			chart.destroy();
		}
	};
};