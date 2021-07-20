init();

async function init() {
    if (typeof process == "object") {
        // We run in the npm/webpack environment.
        const [
            {
                main,
                parse_data_file,
                generate_per_file_stats,
                get_chromosome_names,
                update_plot_class,
                draw_complete_quality_frequency_plot,
                draw_single_chromosome_quality_frequency_plot,
                draw_coverage_per_bin_plot,
                draw_length_of_chromosome_plot,
                draw_covered_length_of_chromosome_plot,
                draw_number_of_reads_per_chromosome_plot,
                draw_coverage_of_chromosomes_plot,
                draw_median_length_of_reads_per_chromosome_plot,
                draw_mean_length_of_reads_per_chromosome_plot,
                draw_mode_length_of_reads_per_chromosome_plot,
                draw_shortest_length_of_reads_per_chromosome_plot,
                draw_longest_length_of_reads_per_chromosome_plot
            },
            {
                setup
            }
        ] = await Promise.all([

            import("segemehl_21_frontend"),
            import("./index.js"),
        ]);
        setup(
            parse_data_file,
            generate_per_file_stats,
            get_chromosome_names,
            update_plot_class,
            draw_complete_quality_frequency_plot,
            draw_single_chromosome_quality_frequency_plot,
            draw_coverage_per_bin_plot,
            draw_length_of_chromosome_plot,
            draw_covered_length_of_chromosome_plot,
            draw_number_of_reads_per_chromosome_plot,
            draw_coverage_of_chromosomes_plot,
            draw_median_length_of_reads_per_chromosome_plot,
            draw_mean_length_of_reads_per_chromosome_plot,
            draw_mode_length_of_reads_per_chromosome_plot,
            draw_shortest_length_of_reads_per_chromosome_plot,
            draw_longest_length_of_reads_per_chromosome_plot
        );
        main();
    } else {
        const [
            {
                default: init,
                main,
                parse_data_file,
                generate_per_file_stats,
                get_chromosome_names,
                update_plot_class,
                draw_complete_quality_frequency_plot,
                draw_single_chromosome_quality_frequency_plot,
                draw_coverage_per_bin_plot,
                draw_length_of_chromosome_plot,
                draw_covered_length_of_chromosome_plot,
                draw_number_of_reads_per_chromosome_plot,
                draw_coverage_of_chromosomes_plot,
                draw_median_length_of_reads_per_chromosome_plot,
                draw_mean_length_of_reads_per_chromosome_plot,
                draw_mode_length_of_reads_per_chromosome_plot,
                draw_shortest_length_of_reads_per_chromosome_plot,
                draw_longest_length_of_reads_per_chromosome_plot
            },
            {
                setup
            }
        ] = await Promise.all([
            import("../pkg/segemehl_21_frontend.js"),
            import("./index.js"),
        ]);
        await init();
        setup(
            parse_data_file,
            generate_per_file_stats,
            get_chromosome_names,
            update_plot_class,
            draw_complete_quality_frequency_plot,
            draw_single_chromosome_quality_frequency_plot,
            draw_coverage_per_bin_plot,
            draw_length_of_chromosome_plot,
            draw_covered_length_of_chromosome_plot,
            draw_number_of_reads_per_chromosome_plot,
            draw_coverage_of_chromosomes_plot,
            draw_median_length_of_reads_per_chromosome_plot,
            draw_mean_length_of_reads_per_chromosome_plot,
            draw_mode_length_of_reads_per_chromosome_plot,
            draw_shortest_length_of_reads_per_chromosome_plot,
            draw_longest_length_of_reads_per_chromosome_plot
        );
        main();
    }
}