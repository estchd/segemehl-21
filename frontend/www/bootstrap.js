init();

async function init() {
    if (typeof process == "object") {
        // We run in the npm/webpack environment.
        const [
            {
                main,
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
                draw_longest_length_of_reads_per_chromosome_plot,
                setup_file_list,
                add_file,
                process_file,
                remove_file,
                get_file_list
            },
            {
                setup
            }
        ] = await Promise.all([
            import("/../pkg"),
            import("./index"),
        ]);
        setup(
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
            draw_longest_length_of_reads_per_chromosome_plot,
            setup_file_list,
            add_file,
            process_file,
            remove_file,
            get_file_list
        );
        main();
    } else {
        const [
            {
                default: init,
                main,
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
                draw_longest_length_of_reads_per_chromosome_plot,
                setup_file_list,
                add_file,
                process_file,
                remove_file,
                get_file_list
            },
            {
                setup
            }
        ] = await Promise.all([
            import("/../pkg"),
            import("./index"),
        ]);
        await init();
        setup(
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
            draw_longest_length_of_reads_per_chromosome_plot,
            setup_file_list,
            add_file,
            process_file,
            remove_file,
            get_file_list
        );
        main();
    }
}