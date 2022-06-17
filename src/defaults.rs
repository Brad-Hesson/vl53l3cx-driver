#![allow(clippy::derivable_impls)]
use crate::bindings;

impl Default for bindings::VL53LX_AdditionalData_t {
    fn default() -> Self {
        Self {
            preset_mode: Default::default(),
            zone_preset: Default::default(),
            measurement_mode: Default::default(),
            offset_calibration_mode: Default::default(),
            offset_correction_mode: Default::default(),
            dmax_mode: Default::default(),
            phasecal_config_timeout_us: Default::default(),
            mm_config_timeout_us: Default::default(),
            range_config_timeout_us: Default::default(),
            inter_measurement_period_ms: Default::default(),
            dss_config__target_total_rate_mcps: Default::default(),
            VL53LX_p_006: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_DeviceInfo_t {
    fn default() -> Self {
        Self {
            ProductType: Default::default(),
            ProductRevisionMajor: Default::default(),
            ProductRevisionMinor: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_Version_t {
    fn default() -> Self {
        Self {
            revision: Default::default(),
            major: Default::default(),
            minor: Default::default(),
            build: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_MultiRangingData_t {
    fn default() -> Self {
        Self {
            TimeStamp: Default::default(),
            StreamCount: Default::default(),
            NumberOfObjectsFound: Default::default(),
            RangeData: Default::default(),
            HasXtalkValueChanged: Default::default(),
            EffectiveSpadRtnCount: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_TargetRangeData_t {
    fn default() -> Self {
        Self {
            RangeMaxMilliMeter: Default::default(),
            RangeMinMilliMeter: Default::default(),
            SignalRateRtnMegaCps: Default::default(),
            AmbientRateRtnMegaCps: Default::default(),
            SigmaMilliMeter: Default::default(),
            RangeMilliMeter: Default::default(),
            RangeStatus: Default::default(),
            ExtendedRange: Default::default(),
        }
    }
}

impl Default for bindings::VL53LX_DevData_t {
    fn default() -> Self {
        Self {
            LLData: Default::default(),
            llresults: Default::default(),
            CurrentParameters: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_DeviceParameters_t {
    fn default() -> Self {
        Self {
            DistanceMode: Default::default(),
            MeasurementTimingBudgetMicroSeconds: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_LLDriverResults_t {
    fn default() -> Self {
        Self {
            range_results: Default::default(),
            zone_dyn_cfgs: Default::default(),
            zone_results: Default::default(),
            zone_hists: Default::default(),
            zone_cal: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zone_calibration_results_t {
    fn default() -> Self {
        Self {
            struct_version: Default::default(),
            preset_mode: Default::default(),
            zone_preset: Default::default(),
            cal_distance_mm: Default::default(),
            cal_reflectance_pc: Default::default(),
            phasecal_result__reference_phase: Default::default(),
            zero_distance_phase: Default::default(),
            cal_status: Default::default(),
            max_zones: Default::default(),
            active_zones: Default::default(),
            VL53LX_p_003: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zone_calibration_data_t {
    fn default() -> Self {
        Self {
            no_of_samples: Default::default(),
            effective_spads: Default::default(),
            peak_rate_mcps: Default::default(),
            VL53LX_p_011: Default::default(),
            VL53LX_p_002: Default::default(),
            median_range_mm: Default::default(),
            range_mm_offset: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zone_histograms_t {
    fn default() -> Self {
        Self {
            max_zones: Default::default(),
            active_zones: Default::default(),
            VL53LX_p_003: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zone_hist_info_t {
    fn default() -> Self {
        Self {
            rd_device_state: Default::default(),
            number_of_ambient_bins: Default::default(),
            result__dss_actual_effective_spads: Default::default(),
            VL53LX_p_005: Default::default(),
            total_periods_elapsed: Default::default(),
            ambient_events_sum: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zone_results_t {
    fn default() -> Self {
        Self {
            max_zones: Default::default(),
            active_zones: Default::default(),
            VL53LX_p_003: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zone_objects_t {
    fn default() -> Self {
        Self {
            cfg_device_state: Default::default(),
            rd_device_state: Default::default(),
            zone_id: Default::default(),
            stream_count: Default::default(),
            max_objects: Default::default(),
            active_objects: Default::default(),
            VL53LX_p_003: Default::default(),
            xmonitor: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_object_data_t {
    fn default() -> Self {
        Self {
            VL53LX_p_016: Default::default(),
            VL53LX_p_017: Default::default(),
            VL53LX_p_011: Default::default(),
            range_status: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zone_private_dyn_cfgs_t {
    fn default() -> Self {
        Self {
            max_zones: Default::default(),
            active_zones: Default::default(),
            VL53LX_p_003: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zone_private_dyn_cfg_t {
    fn default() -> Self {
        Self {
            expected_stream_count: Default::default(),
            expected_gph_id: Default::default(),
            dss_mode: Default::default(),
            dss_requested_effective_spad_count: Default::default(),
            seed_cfg: Default::default(),
            initial_phase_seed: Default::default(),
            roi_config__user_roi_centre_spad: Default::default(),
            roi_config__user_roi_requested_global_xy_size: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_range_results_t {
    fn default() -> Self {
        Self {
            cfg_device_state: Default::default(),
            rd_device_state: Default::default(),
            zone_id: Default::default(),
            stream_count: Default::default(),
            VL53LX_p_022: Default::default(),
            wrap_dmax_mm: Default::default(),
            device_status: Default::default(),
            max_results: Default::default(),
            active_results: Default::default(),
            VL53LX_p_003: Default::default(),
            xmonitor: Default::default(),
            smudge_corrector_data: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_smudge_corrector_data_t {
    fn default() -> Self {
        Self {
            smudge_corr_valid: Default::default(),
            smudge_corr_clipped: Default::default(),
            single_xtalk_delta_flag: Default::default(),
            averaged_xtalk_delta_flag: Default::default(),
            sample_limit_exceeded_flag: Default::default(),
            gradient_zero_flag: Default::default(),
            new_xtalk_applied_flag: Default::default(),
            algo__crosstalk_compensation_plane_offset_kcps: Default::default(),
            algo__crosstalk_compensation_x_plane_gradient_kcps: Default::default(),
            algo__crosstalk_compensation_y_plane_gradient_kcps: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_range_data_t {
    fn default() -> Self {
        Self {
            range_id: Default::default(),
            time_stamp: Default::default(),
            VL53LX_p_012: Default::default(),
            VL53LX_p_019: Default::default(),
            VL53LX_p_023: Default::default(),
            VL53LX_p_024: Default::default(),
            VL53LX_p_013: Default::default(),
            VL53LX_p_025: Default::default(),
            width: Default::default(),
            VL53LX_p_029: Default::default(),
            fast_osc_frequency: Default::default(),
            zero_distance_phase: Default::default(),
            VL53LX_p_004: Default::default(),
            total_periods_elapsed: Default::default(),
            peak_duration_us: Default::default(),
            woi_duration_us: Default::default(),
            VL53LX_p_016: Default::default(),
            VL53LX_p_017: Default::default(),
            VL53LX_p_010: Default::default(),
            peak_signal_count_rate_mcps: Default::default(),
            avg_signal_count_rate_mcps: Default::default(),
            ambient_count_rate_mcps: Default::default(),
            total_rate_per_spad_mcps: Default::default(),
            VL53LX_p_009: Default::default(),
            VL53LX_p_002: Default::default(),
            VL53LX_p_026: Default::default(),
            VL53LX_p_011: Default::default(),
            VL53LX_p_027: Default::default(),
            min_range_mm: Default::default(),
            median_range_mm: Default::default(),
            max_range_mm: Default::default(),
            range_status: Default::default(),
        }
    }
}

impl Default for bindings::VL53LX_LLDriverData_t {
    fn default() -> Self {
        Self {
            wait_method: Default::default(),
            preset_mode: Default::default(),
            zone_preset: Default::default(),
            measurement_mode: Default::default(),
            offset_calibration_mode: Default::default(),
            offset_correction_mode: Default::default(),
            dmax_mode: Default::default(),
            phasecal_config_timeout_us: Default::default(),
            mm_config_timeout_us: Default::default(),
            range_config_timeout_us: Default::default(),
            inter_measurement_period_ms: Default::default(),
            dss_config__target_total_rate_mcps: Default::default(),
            fw_ready_poll_duration_ms: Default::default(),
            fw_ready: Default::default(),
            debug_mode: Default::default(),
            version: Default::default(),
            ll_state: Default::default(),
            gpio_interrupt_config: Default::default(),
            customer: Default::default(),
            cal_peak_rate_map: Default::default(),
            add_off_cal_data: Default::default(),
            fmt_dmax_cal: Default::default(),
            cust_dmax_cal: Default::default(),
            gain_cal: Default::default(),
            mm_roi: Default::default(),
            optical_centre: Default::default(),
            zone_cfg: Default::default(),
            tuning_parms: Default::default(),
            rtn_good_spads: Default::default(),
            refspadchar: Default::default(),
            ssc_cfg: Default::default(),
            histpostprocess: Default::default(),
            dmax_cfg: Default::default(),
            xtalk_extract_cfg: Default::default(),
            xtalk_cfg: Default::default(),
            offsetcal_cfg: Default::default(),
            zonecal_cfg: Default::default(),
            stat_nvm: Default::default(),
            hist_cfg: Default::default(),
            stat_cfg: Default::default(),
            gen_cfg: Default::default(),
            tim_cfg: Default::default(),
            dyn_cfg: Default::default(),
            sys_ctrl: Default::default(),
            sys_results: Default::default(),
            nvm_copy_data: Default::default(),
            hist_data: Default::default(),
            hist_xtalk: Default::default(),
            xtalk_shapes: Default::default(),
            xtalk_results: Default::default(),
            xtalk_cal: Default::default(),
            xtalk_extract: Default::default(),
            offset_results: Default::default(),
            core_results: Default::default(),
            dbg_results: Default::default(),
            smudge_correct_config: Default::default(),
            smudge_corrector_internals: Default::default(),
            low_power_auto_data: Default::default(),
            wArea1: [0; 1536],
            wArea2: [0; 512],
            per_vcsel_cal_data: Default::default(),
            bin_rec_pos: Default::default(),
            pos_before_next_recom: Default::default(),
            multi_bins_rec: Default::default(),
            PreviousRangeMilliMeter: Default::default(),
            PreviousRangeStatus: Default::default(),
            PreviousExtendedRange: Default::default(),
            PreviousStreamCount: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_per_vcsel_period_offset_cal_data_t {
    fn default() -> Self {
        Self {
            short_a_offset_mm: Default::default(),
            short_b_offset_mm: Default::default(),
            medium_a_offset_mm: Default::default(),
            medium_b_offset_mm: Default::default(),
            long_a_offset_mm: Default::default(),
            long_b_offset_mm: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_low_power_auto_data_t {
    fn default() -> Self {
        Self {
            vhv_loop_bound: Default::default(),
            is_low_power_auto_mode: Default::default(),
            low_power_auto_range_count: Default::default(),
            saved_interrupt_config: Default::default(),
            saved_vhv_init: Default::default(),
            saved_vhv_timeout: Default::default(),
            first_run_phasecal_result: Default::default(),
            dss__total_rate_per_spad_mcps: Default::default(),
            dss__required_spads: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_smudge_corrector_internals_t {
    fn default() -> Self {
        Self {
            current_samples: Default::default(),
            required_samples: Default::default(),
            accumulator: Default::default(),
            nodetect_counter: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_smudge_corrector_config_t {
    fn default() -> Self {
        Self {
            smudge_corr_enabled: Default::default(),
            smudge_corr_apply_enabled: Default::default(),
            smudge_corr_single_apply: Default::default(),
            smudge_margin: Default::default(),
            noise_margin: Default::default(),
            user_xtalk_offset_limit: Default::default(),
            user_xtalk_offset_limit_hi: Default::default(),
            sample_limit: Default::default(),
            single_xtalk_delta: Default::default(),
            averaged_xtalk_delta: Default::default(),
            smudge_corr_clip_limit: Default::default(),
            smudge_corr_ambient_threshold: Default::default(),
            scaler_calc_method: Default::default(),
            x_gradient_scaler: Default::default(),
            y_gradient_scaler: Default::default(),
            user_scaler_set: Default::default(),
            nodetect_ambient_threshold: Default::default(),
            nodetect_sample_limit: Default::default(),
            nodetect_xtalk_offset: Default::default(),
            nodetect_min_range_mm: Default::default(),
            max_smudge_factor: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_debug_results_t {
    fn default() -> Self {
        Self {
            phasecal_result__reference_phase: Default::default(),
            phasecal_result__vcsel_start: Default::default(),
            ref_spad_char_result__num_actual_ref_spads: Default::default(),
            ref_spad_char_result__ref_location: Default::default(),
            vhv_result__coldboot_status: Default::default(),
            vhv_result__search_result: Default::default(),
            vhv_result__latest_setting: Default::default(),
            result__osc_calibrate_val: Default::default(),
            ana_config__powerdown_go1: Default::default(),
            ana_config__ref_bg_ctrl: Default::default(),
            ana_config__regdvdd1v2_ctrl: Default::default(),
            ana_config__osc_slow_ctrl: Default::default(),
            test_mode__status: Default::default(),
            firmware__system_status: Default::default(),
            firmware__mode_status: Default::default(),
            firmware__secondary_mode_status: Default::default(),
            firmware__cal_repeat_rate_counter: Default::default(),
            gph__system__thresh_high: Default::default(),
            gph__system__thresh_low: Default::default(),
            gph__system__enable_xtalk_per_quadrant: Default::default(),
            gph__spare_0: Default::default(),
            gph__sd_config__woi_sd0: Default::default(),
            gph__sd_config__woi_sd1: Default::default(),
            gph__sd_config__initial_phase_sd0: Default::default(),
            gph__sd_config__initial_phase_sd1: Default::default(),
            gph__sd_config__first_order_select: Default::default(),
            gph__sd_config__quantifier: Default::default(),
            gph__roi_config__user_roi_centre_spad: Default::default(),
            gph__roi_config__user_roi_requested_global_xy_size: Default::default(),
            gph__system__sequence_config: Default::default(),
            gph__gph_id: Default::default(),
            system__interrupt_set: Default::default(),
            interrupt_manager__enables: Default::default(),
            interrupt_manager__clear: Default::default(),
            interrupt_manager__status: Default::default(),
            mcu_to_host_bank__wr_access_en: Default::default(),
            power_management__go1_reset_status: Default::default(),
            pad_startup_mode__value_ro: Default::default(),
            pad_startup_mode__value_ctrl: Default::default(),
            pll_period_us: Default::default(),
            interrupt_scheduler__data_out: Default::default(),
            nvm_bist__complete: Default::default(),
            nvm_bist__status: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_core_results_t {
    fn default() -> Self {
        Self {
            result_core__ambient_window_events_sd0: Default::default(),
            result_core__ranging_total_events_sd0: Default::default(),
            result_core__signal_total_events_sd0: Default::default(),
            result_core__total_periods_elapsed_sd0: Default::default(),
            result_core__ambient_window_events_sd1: Default::default(),
            result_core__ranging_total_events_sd1: Default::default(),
            result_core__signal_total_events_sd1: Default::default(),
            result_core__total_periods_elapsed_sd1: Default::default(),
            result_core__spare_0: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_offset_range_results_t {
    fn default() -> Self {
        Self {
            cal_distance_mm: Default::default(),
            cal_reflectance_pc: Default::default(),
            cal_status: Default::default(),
            cal_report: Default::default(),
            max_results: Default::default(),
            active_results: Default::default(),
            VL53LX_p_003: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_offset_range_data_t {
    fn default() -> Self {
        Self {
            preset_mode: Default::default(),
            dss_config__roi_mode_control: Default::default(),
            dss_config__manual_effective_spads_select: Default::default(),
            no_of_samples: Default::default(),
            effective_spads: Default::default(),
            peak_rate_mcps: Default::default(),
            VL53LX_p_002: Default::default(),
            median_range_mm: Default::default(),
            range_mm_offset: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_hist_xtalk_extract_data_t {
    fn default() -> Self {
        Self {
            sample_count: Default::default(),
            pll_period_mm: Default::default(),
            peak_duration_us_sum: Default::default(),
            effective_spad_count_sum: Default::default(),
            zero_distance_phase_sum: Default::default(),
            zero_distance_phase_avg: Default::default(),
            event_scaler_sum: Default::default(),
            event_scaler_avg: Default::default(),
            signal_events_sum: Default::default(),
            xtalk_rate_kcps_per_spad: Default::default(),
            xtalk_start_phase: Default::default(),
            xtalk_end_phase: Default::default(),
            xtalk_width_phase: Default::default(),
            target_start_phase: Default::default(),
            target_end_phase: Default::default(),
            target_width_phase: Default::default(),
            effective_width: Default::default(),
            event_scaler: Default::default(),
            VL53LX_p_012: Default::default(),
            VL53LX_p_013: Default::default(),
            target_start: Default::default(),
            max_shape_value: Default::default(),
            bin_data_sums: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_xtalk_calibration_results_t {
    fn default() -> Self {
        Self {
            algo__crosstalk_compensation_plane_offset_kcps: Default::default(),
            algo__crosstalk_compensation_x_plane_gradient_kcps: Default::default(),
            algo__crosstalk_compensation_y_plane_gradient_kcps: Default::default(),
            algo__xtalk_cpo_HistoMerge_kcps: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_xtalk_range_results_t {
    fn default() -> Self {
        Self {
            cal_status: Default::default(),
            num_of_samples_status: Default::default(),
            zero_samples_status: Default::default(),
            max_sigma_status: Default::default(),
            max_results: Default::default(),
            active_results: Default::default(),
            VL53LX_p_003: Default::default(),
            central_histogram_sum: Default::default(),
            central_histogram_avg: Default::default(),
            central_histogram__window_start: Default::default(),
            central_histogram__window_end: Default::default(),
            histogram_avg_1: Default::default(),
            histogram_avg_2: Default::default(),
            xtalk_avg: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_xtalk_range_data_t {
    fn default() -> Self {
        Self {
            no_of_samples: Default::default(),
            rate_per_spad_kcps_sum: Default::default(),
            rate_per_spad_kcps_avg: Default::default(),
            signal_total_events_sum: Default::default(),
            signal_total_events_avg: Default::default(),
            sigma_mm_sum: Default::default(),
            sigma_mm_avg: Default::default(),
            median_phase_sum: Default::default(),
            median_phase_avg: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_xtalk_histogram_data_t {
    fn default() -> Self {
        Self {
            xtalk_shape: Default::default(),
            xtalk_hist_removed: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_xtalk_histogram_shape_t {
    fn default() -> Self {
        Self {
            zone_id: Default::default(),
            time_stamp: Default::default(),
            VL53LX_p_019: Default::default(),
            VL53LX_p_020: Default::default(),
            VL53LX_p_021: Default::default(),
            bin_data: Default::default(),
            phasecal_result__reference_phase: Default::default(),
            phasecal_result__vcsel_start: Default::default(),
            cal_config__vcsel_start: Default::default(),
            vcsel_width: Default::default(),
            VL53LX_p_015: Default::default(),
            zero_distance_phase: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_histogram_bin_data_t {
    fn default() -> Self {
        Self {
            cfg_device_state: Default::default(),
            rd_device_state: Default::default(),
            zone_id: Default::default(),
            time_stamp: Default::default(),
            VL53LX_p_019: Default::default(),
            VL53LX_p_020: Default::default(),
            VL53LX_p_021: Default::default(),
            number_of_ambient_bins: Default::default(),
            bin_seq: Default::default(),
            bin_rep: Default::default(),
            bin_data: Default::default(),
            result__interrupt_status: Default::default(),
            result__range_status: Default::default(),
            result__report_status: Default::default(),
            result__stream_count: Default::default(),
            result__dss_actual_effective_spads: Default::default(),
            phasecal_result__reference_phase: Default::default(),
            phasecal_result__vcsel_start: Default::default(),
            cal_config__vcsel_start: Default::default(),
            vcsel_width: Default::default(),
            VL53LX_p_005: Default::default(),
            VL53LX_p_015: Default::default(),
            total_periods_elapsed: Default::default(),
            peak_duration_us: Default::default(),
            woi_duration_us: Default::default(),
            min_bin_value: Default::default(),
            max_bin_value: Default::default(),
            zero_distance_phase: Default::default(),
            number_of_ambient_samples: Default::default(),
            ambient_events_sum: Default::default(),
            VL53LX_p_028: Default::default(),
            roi_config__user_roi_centre_spad: Default::default(),
            roi_config__user_roi_requested_global_xy_size: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_nvm_copy_data_t {
    fn default() -> Self {
        Self {
            identification__model_id: Default::default(),
            identification__module_type: Default::default(),
            identification__revision_id: Default::default(),
            identification__module_id: Default::default(),
            ana_config__fast_osc__trim_max: Default::default(),
            ana_config__fast_osc__freq_set: Default::default(),
            ana_config__vcsel_trim: Default::default(),
            ana_config__vcsel_selion: Default::default(),
            ana_config__vcsel_selion_max: Default::default(),
            protected_laser_safety__lock_bit: Default::default(),
            laser_safety__key: Default::default(),
            laser_safety__key_ro: Default::default(),
            laser_safety__clip: Default::default(),
            laser_safety__mult: Default::default(),
            global_config__spad_enables_rtn_0: Default::default(),
            global_config__spad_enables_rtn_1: Default::default(),
            global_config__spad_enables_rtn_2: Default::default(),
            global_config__spad_enables_rtn_3: Default::default(),
            global_config__spad_enables_rtn_4: Default::default(),
            global_config__spad_enables_rtn_5: Default::default(),
            global_config__spad_enables_rtn_6: Default::default(),
            global_config__spad_enables_rtn_7: Default::default(),
            global_config__spad_enables_rtn_8: Default::default(),
            global_config__spad_enables_rtn_9: Default::default(),
            global_config__spad_enables_rtn_10: Default::default(),
            global_config__spad_enables_rtn_11: Default::default(),
            global_config__spad_enables_rtn_12: Default::default(),
            global_config__spad_enables_rtn_13: Default::default(),
            global_config__spad_enables_rtn_14: Default::default(),
            global_config__spad_enables_rtn_15: Default::default(),
            global_config__spad_enables_rtn_16: Default::default(),
            global_config__spad_enables_rtn_17: Default::default(),
            global_config__spad_enables_rtn_18: Default::default(),
            global_config__spad_enables_rtn_19: Default::default(),
            global_config__spad_enables_rtn_20: Default::default(),
            global_config__spad_enables_rtn_21: Default::default(),
            global_config__spad_enables_rtn_22: Default::default(),
            global_config__spad_enables_rtn_23: Default::default(),
            global_config__spad_enables_rtn_24: Default::default(),
            global_config__spad_enables_rtn_25: Default::default(),
            global_config__spad_enables_rtn_26: Default::default(),
            global_config__spad_enables_rtn_27: Default::default(),
            global_config__spad_enables_rtn_28: Default::default(),
            global_config__spad_enables_rtn_29: Default::default(),
            global_config__spad_enables_rtn_30: Default::default(),
            global_config__spad_enables_rtn_31: Default::default(),
            roi_config__mode_roi_centre_spad: Default::default(),
            roi_config__mode_roi_xy_size: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_system_results_t {
    fn default() -> Self {
        Self {
            result__interrupt_status: Default::default(),
            result__range_status: Default::default(),
            result__report_status: Default::default(),
            result__stream_count: Default::default(),
            result__dss_actual_effective_spads_sd0: Default::default(),
            result__peak_signal_count_rate_mcps_sd0: Default::default(),
            result__ambient_count_rate_mcps_sd0: Default::default(),
            result__sigma_sd0: Default::default(),
            result__phase_sd0: Default::default(),
            result__final_crosstalk_corrected_range_mm_sd0: Default::default(),
            result__peak_signal_count_rate_crosstalk_corrected_mcps_sd0: Default::default(),
            result__mm_inner_actual_effective_spads_sd0: Default::default(),
            result__mm_outer_actual_effective_spads_sd0: Default::default(),
            result__avg_signal_count_rate_mcps_sd0: Default::default(),
            result__dss_actual_effective_spads_sd1: Default::default(),
            result__peak_signal_count_rate_mcps_sd1: Default::default(),
            result__ambient_count_rate_mcps_sd1: Default::default(),
            result__sigma_sd1: Default::default(),
            result__phase_sd1: Default::default(),
            result__final_crosstalk_corrected_range_mm_sd1: Default::default(),
            result__spare_0_sd1: Default::default(),
            result__spare_1_sd1: Default::default(),
            result__spare_2_sd1: Default::default(),
            result__spare_3_sd1: Default::default(),
            result__thresh_info: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_system_control_t {
    fn default() -> Self {
        Self {
            power_management__go1_power_force: Default::default(),
            system__stream_count_ctrl: Default::default(),
            firmware__enable: Default::default(),
            system__interrupt_clear: Default::default(),
            system__mode_start: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_dynamic_config_t {
    fn default() -> Self {
        Self {
            system__grouped_parameter_hold_0: Default::default(),
            system__thresh_high: Default::default(),
            system__thresh_low: Default::default(),
            system__enable_xtalk_per_quadrant: Default::default(),
            system__seed_config: Default::default(),
            sd_config__woi_sd0: Default::default(),
            sd_config__woi_sd1: Default::default(),
            sd_config__initial_phase_sd0: Default::default(),
            sd_config__initial_phase_sd1: Default::default(),
            system__grouped_parameter_hold_1: Default::default(),
            sd_config__first_order_select: Default::default(),
            sd_config__quantifier: Default::default(),
            roi_config__user_roi_centre_spad: Default::default(),
            roi_config__user_roi_requested_global_xy_size: Default::default(),
            system__sequence_config: Default::default(),
            system__grouped_parameter_hold: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_timing_config_t {
    fn default() -> Self {
        Self {
            mm_config__timeout_macrop_a_hi: Default::default(),
            mm_config__timeout_macrop_a_lo: Default::default(),
            mm_config__timeout_macrop_b_hi: Default::default(),
            mm_config__timeout_macrop_b_lo: Default::default(),
            range_config__timeout_macrop_a_hi: Default::default(),
            range_config__timeout_macrop_a_lo: Default::default(),
            range_config__vcsel_period_a: Default::default(),
            range_config__timeout_macrop_b_hi: Default::default(),
            range_config__timeout_macrop_b_lo: Default::default(),
            range_config__vcsel_period_b: Default::default(),
            range_config__sigma_thresh: Default::default(),
            range_config__min_count_rate_rtn_limit_mcps: Default::default(),
            range_config__valid_phase_low: Default::default(),
            range_config__valid_phase_high: Default::default(),
            system__intermeasurement_period: Default::default(),
            system__fractional_enable: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_general_config_t {
    fn default() -> Self {
        Self {
            gph_config__stream_count_update_value: Default::default(),
            global_config__stream_divider: Default::default(),
            system__interrupt_config_gpio: Default::default(),
            cal_config__vcsel_start: Default::default(),
            cal_config__repeat_rate: Default::default(),
            global_config__vcsel_width: Default::default(),
            phasecal_config__timeout_macrop: Default::default(),
            phasecal_config__target: Default::default(),
            phasecal_config__override: Default::default(),
            dss_config__roi_mode_control: Default::default(),
            system__thresh_rate_high: Default::default(),
            system__thresh_rate_low: Default::default(),
            dss_config__manual_effective_spads_select: Default::default(),
            dss_config__manual_block_select: Default::default(),
            dss_config__aperture_attenuation: Default::default(),
            dss_config__max_spads_limit: Default::default(),
            dss_config__min_spads_limit: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_static_config_t {
    fn default() -> Self {
        Self {
            dss_config__target_total_rate_mcps: Default::default(),
            debug__ctrl: Default::default(),
            test_mode__ctrl: Default::default(),
            clk_gating__ctrl: Default::default(),
            nvm_bist__ctrl: Default::default(),
            nvm_bist__num_nvm_words: Default::default(),
            nvm_bist__start_address: Default::default(),
            host_if__status: Default::default(),
            pad_i2c_hv__config: Default::default(),
            pad_i2c_hv__extsup_config: Default::default(),
            gpio_hv_pad__ctrl: Default::default(),
            gpio_hv_mux__ctrl: Default::default(),
            gpio__tio_hv_status: Default::default(),
            gpio__fio_hv_status: Default::default(),
            ana_config__spad_sel_pswidth: Default::default(),
            ana_config__vcsel_pulse_width_offset: Default::default(),
            ana_config__fast_osc__config_ctrl: Default::default(),
            sigma_estimator__effective_pulse_width_ns: Default::default(),
            sigma_estimator__effective_ambient_width_ns: Default::default(),
            sigma_estimator__sigma_ref_mm: Default::default(),
            algo__crosstalk_compensation_valid_height_mm: Default::default(),
            spare_host_config__static_config_spare_0: Default::default(),
            spare_host_config__static_config_spare_1: Default::default(),
            algo__range_ignore_threshold_mcps: Default::default(),
            algo__range_ignore_valid_height_mm: Default::default(),
            algo__range_min_clip: Default::default(),
            algo__consistency_check__tolerance: Default::default(),
            spare_host_config__static_config_spare_2: Default::default(),
            sd_config__reset_stages_msb: Default::default(),
            sd_config__reset_stages_lsb: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_histogram_config_t {
    fn default() -> Self {
        Self {
            histogram_config__spad_array_selection: Default::default(),
            histogram_config__low_amb_even_bin_0_1: Default::default(),
            histogram_config__low_amb_even_bin_2_3: Default::default(),
            histogram_config__low_amb_even_bin_4_5: Default::default(),
            histogram_config__low_amb_odd_bin_0_1: Default::default(),
            histogram_config__low_amb_odd_bin_2_3: Default::default(),
            histogram_config__low_amb_odd_bin_4_5: Default::default(),
            histogram_config__mid_amb_even_bin_0_1: Default::default(),
            histogram_config__mid_amb_even_bin_2_3: Default::default(),
            histogram_config__mid_amb_even_bin_4_5: Default::default(),
            histogram_config__mid_amb_odd_bin_0_1: Default::default(),
            histogram_config__mid_amb_odd_bin_2: Default::default(),
            histogram_config__mid_amb_odd_bin_3_4: Default::default(),
            histogram_config__mid_amb_odd_bin_5: Default::default(),
            histogram_config__user_bin_offset: Default::default(),
            histogram_config__high_amb_even_bin_0_1: Default::default(),
            histogram_config__high_amb_even_bin_2_3: Default::default(),
            histogram_config__high_amb_even_bin_4_5: Default::default(),
            histogram_config__high_amb_odd_bin_0_1: Default::default(),
            histogram_config__high_amb_odd_bin_2_3: Default::default(),
            histogram_config__high_amb_odd_bin_4_5: Default::default(),
            histogram_config__amb_thresh_low: Default::default(),
            histogram_config__amb_thresh_high: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_static_nvm_managed_t {
    fn default() -> Self {
        Self {
            i2c_slave__device_address: Default::default(),
            ana_config__vhv_ref_sel_vddpix: Default::default(),
            ana_config__vhv_ref_sel_vquench: Default::default(),
            ana_config__reg_avdd1v2_sel: Default::default(),
            ana_config__fast_osc__trim: Default::default(),
            osc_measured__fast_osc__frequency: Default::default(),
            vhv_config__timeout_macrop_loop_bound: Default::default(),
            vhv_config__count_thresh: Default::default(),
            vhv_config__offset: Default::default(),
            vhv_config__init: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zonecal_config_t {
    fn default() -> Self {
        Self {
            dss_config__target_total_rate_mcps: Default::default(),
            phasecal_config_timeout_us: Default::default(),
            mm_config_timeout_us: Default::default(),
            range_config_timeout_us: Default::default(),
            phasecal_num_of_samples: Default::default(),
            zone_num_of_samples: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_offsetcal_config_t {
    fn default() -> Self {
        Self {
            dss_config__target_total_rate_mcps: Default::default(),
            phasecal_config_timeout_us: Default::default(),
            range_config_timeout_us: Default::default(),
            mm_config_timeout_us: Default::default(),
            pre_num_of_samples: Default::default(),
            mm1_num_of_samples: Default::default(),
            mm2_num_of_samples: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_xtalk_config_t {
    fn default() -> Self {
        Self {
            algo__crosstalk_compensation_plane_offset_kcps: Default::default(),
            algo__crosstalk_compensation_x_plane_gradient_kcps: Default::default(),
            algo__crosstalk_compensation_y_plane_gradient_kcps: Default::default(),
            nvm_default__crosstalk_compensation_plane_offset_kcps: Default::default(),
            nvm_default__crosstalk_compensation_x_plane_gradient_kcps: Default::default(),
            nvm_default__crosstalk_compensation_y_plane_gradient_kcps: Default::default(),
            global_crosstalk_compensation_enable: Default::default(),
            histogram_mode_crosstalk_margin_kcps: Default::default(),
            lite_mode_crosstalk_margin_kcps: Default::default(),
            crosstalk_range_ignore_threshold_mult: Default::default(),
            crosstalk_range_ignore_threshold_rate_mcps: Default::default(),
            algo__crosstalk_detect_min_valid_range_mm: Default::default(),
            algo__crosstalk_detect_max_valid_range_mm: Default::default(),
            algo__crosstalk_detect_max_valid_rate_kcps: Default::default(),
            algo__crosstalk_detect_max_sigma_mm: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_xtalkextract_config_t {
    fn default() -> Self {
        Self {
            dss_config__target_total_rate_mcps: Default::default(),
            phasecal_config_timeout_us: Default::default(),
            mm_config_timeout_us: Default::default(),
            range_config_timeout_us: Default::default(),
            num_of_samples: Default::default(),
            algo__crosstalk_extract_min_valid_range_mm: Default::default(),
            algo__crosstalk_extract_max_valid_range_mm: Default::default(),
            algo__crosstalk_extract_max_valid_rate_kcps: Default::default(),
            algo__crosstalk_extract_max_sigma_mm: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_hist_gen3_dmax_config_t {
    fn default() -> Self {
        Self {
            signal_thresh_sigma: Default::default(),
            ambient_thresh_sigma: Default::default(),
            min_ambient_thresh_events: Default::default(),
            signal_total_events_limit: Default::default(),
            target_reflectance_for_dmax_calc: Default::default(),
            max_effective_spads: Default::default(),
            dss_config__target_total_rate_mcps: Default::default(),
            dss_config__aperture_attenuation: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_hist_post_process_config_t {
    fn default() -> Self {
        Self {
            hist_algo_select: Default::default(),
            hist_target_order: Default::default(),
            filter_woi0: Default::default(),
            filter_woi1: Default::default(),
            hist_amb_est_method: Default::default(),
            ambient_thresh_sigma0: Default::default(),
            ambient_thresh_sigma1: Default::default(),
            ambient_thresh_events_scaler: Default::default(),
            min_ambient_thresh_events: Default::default(),
            noise_threshold: Default::default(),
            signal_total_events_limit: Default::default(),
            sigma_estimator__sigma_ref_mm: Default::default(),
            sigma_thresh: Default::default(),
            range_offset_mm: Default::default(),
            gain_factor: Default::default(),
            valid_phase_low: Default::default(),
            valid_phase_high: Default::default(),
            algo__consistency_check__phase_tolerance: Default::default(),
            algo__consistency_check__event_sigma: Default::default(),
            algo__consistency_check__event_min_spad_count: Default::default(),
            algo__consistency_check__min_max_tolerance: Default::default(),
            algo__crosstalk_compensation_enable: Default::default(),
            algo__crosstalk_compensation_plane_offset_kcps: Default::default(),
            algo__crosstalk_compensation_x_plane_gradient_kcps: Default::default(),
            algo__crosstalk_compensation_y_plane_gradient_kcps: Default::default(),
            algo__crosstalk_detect_min_valid_range_mm: Default::default(),
            algo__crosstalk_detect_max_valid_range_mm: Default::default(),
            algo__crosstalk_detect_max_valid_rate_kcps: Default::default(),
            algo__crosstalk_detect_max_sigma_mm: Default::default(),
            algo__crosstalk_detect_event_sigma: Default::default(),
            algo__crosstalk_detect_min_max_tolerance: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_ssc_config_t {
    fn default() -> Self {
        Self {
            array_select: Default::default(),
            VL53LX_p_005: Default::default(),
            vcsel_start: Default::default(),
            vcsel_width: Default::default(),
            timeout_us: Default::default(),
            rate_limit_mcps: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_refspadchar_config_t {
    fn default() -> Self {
        Self {
            device_test_mode: Default::default(),
            VL53LX_p_005: Default::default(),
            timeout_us: Default::default(),
            target_count_rate_mcps: Default::default(),
            min_count_rate_limit_mcps: Default::default(),
            max_count_rate_limit_mcps: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_tuning_parm_storage_t {
    fn default() -> Self {
        Self {
            tp_tuning_parm_version: Default::default(),
            tp_tuning_parm_key_table_version: Default::default(),
            tp_tuning_parm_lld_version: Default::default(),
            tp_init_phase_rtn_lite_long: Default::default(),
            tp_init_phase_rtn_lite_med: Default::default(),
            tp_init_phase_rtn_lite_short: Default::default(),
            tp_init_phase_ref_lite_long: Default::default(),
            tp_init_phase_ref_lite_med: Default::default(),
            tp_init_phase_ref_lite_short: Default::default(),
            tp_init_phase_rtn_hist_long: Default::default(),
            tp_init_phase_rtn_hist_med: Default::default(),
            tp_init_phase_rtn_hist_short: Default::default(),
            tp_init_phase_ref_hist_long: Default::default(),
            tp_init_phase_ref_hist_med: Default::default(),
            tp_init_phase_ref_hist_short: Default::default(),
            tp_consistency_lite_phase_tolerance: Default::default(),
            tp_phasecal_target: Default::default(),
            tp_cal_repeat_rate: Default::default(),
            tp_lite_min_clip: Default::default(),
            tp_lite_long_sigma_thresh_mm: Default::default(),
            tp_lite_med_sigma_thresh_mm: Default::default(),
            tp_lite_short_sigma_thresh_mm: Default::default(),
            tp_lite_long_min_count_rate_rtn_mcps: Default::default(),
            tp_lite_med_min_count_rate_rtn_mcps: Default::default(),
            tp_lite_short_min_count_rate_rtn_mcps: Default::default(),
            tp_lite_sigma_est_pulse_width_ns: Default::default(),
            tp_lite_sigma_est_amb_width_ns: Default::default(),
            tp_lite_sigma_ref_mm: Default::default(),
            tp_lite_seed_cfg: Default::default(),
            tp_timed_seed_cfg: Default::default(),
            tp_lite_quantifier: Default::default(),
            tp_lite_first_order_select: Default::default(),
            tp_dss_target_lite_mcps: Default::default(),
            tp_dss_target_histo_mcps: Default::default(),
            tp_dss_target_histo_mz_mcps: Default::default(),
            tp_dss_target_timed_mcps: Default::default(),
            tp_dss_target_very_short_mcps: Default::default(),
            tp_phasecal_timeout_lite_us: Default::default(),
            tp_phasecal_timeout_hist_long_us: Default::default(),
            tp_phasecal_timeout_hist_med_us: Default::default(),
            tp_phasecal_timeout_hist_short_us: Default::default(),
            tp_phasecal_timeout_mz_long_us: Default::default(),
            tp_phasecal_timeout_mz_med_us: Default::default(),
            tp_phasecal_timeout_mz_short_us: Default::default(),
            tp_phasecal_timeout_timed_us: Default::default(),
            tp_mm_timeout_lite_us: Default::default(),
            tp_mm_timeout_histo_us: Default::default(),
            tp_mm_timeout_mz_us: Default::default(),
            tp_mm_timeout_timed_us: Default::default(),
            tp_mm_timeout_lpa_us: Default::default(),
            tp_range_timeout_lite_us: Default::default(),
            tp_range_timeout_histo_us: Default::default(),
            tp_range_timeout_mz_us: Default::default(),
            tp_range_timeout_timed_us: Default::default(),
            tp_range_timeout_lpa_us: Default::default(),
            tp_phasecal_patch_power: Default::default(),
            tp_hist_merge: Default::default(),
            tp_reset_merge_threshold: Default::default(),
            tp_hist_merge_max_size: Default::default(),
            tp_uwr_enable: Default::default(),
            tp_uwr_med_z_1_min: Default::default(),
            tp_uwr_med_z_1_max: Default::default(),
            tp_uwr_med_z_2_min: Default::default(),
            tp_uwr_med_z_2_max: Default::default(),
            tp_uwr_med_z_3_min: Default::default(),
            tp_uwr_med_z_3_max: Default::default(),
            tp_uwr_med_z_4_min: Default::default(),
            tp_uwr_med_z_4_max: Default::default(),
            tp_uwr_med_z_5_min: Default::default(),
            tp_uwr_med_z_5_max: Default::default(),
            tp_uwr_med_corr_z_1_rangea: Default::default(),
            tp_uwr_med_corr_z_1_rangeb: Default::default(),
            tp_uwr_med_corr_z_2_rangea: Default::default(),
            tp_uwr_med_corr_z_2_rangeb: Default::default(),
            tp_uwr_med_corr_z_3_rangea: Default::default(),
            tp_uwr_med_corr_z_3_rangeb: Default::default(),
            tp_uwr_med_corr_z_4_rangea: Default::default(),
            tp_uwr_med_corr_z_4_rangeb: Default::default(),
            tp_uwr_med_corr_z_5_rangea: Default::default(),
            tp_uwr_med_corr_z_5_rangeb: Default::default(),
            tp_uwr_lng_z_1_min: Default::default(),
            tp_uwr_lng_z_1_max: Default::default(),
            tp_uwr_lng_z_2_min: Default::default(),
            tp_uwr_lng_z_2_max: Default::default(),
            tp_uwr_lng_z_3_min: Default::default(),
            tp_uwr_lng_z_3_max: Default::default(),
            tp_uwr_lng_z_4_min: Default::default(),
            tp_uwr_lng_z_4_max: Default::default(),
            tp_uwr_lng_z_5_min: Default::default(),
            tp_uwr_lng_z_5_max: Default::default(),
            tp_uwr_lng_corr_z_1_rangea: Default::default(),
            tp_uwr_lng_corr_z_1_rangeb: Default::default(),
            tp_uwr_lng_corr_z_2_rangea: Default::default(),
            tp_uwr_lng_corr_z_2_rangeb: Default::default(),
            tp_uwr_lng_corr_z_3_rangea: Default::default(),
            tp_uwr_lng_corr_z_3_rangeb: Default::default(),
            tp_uwr_lng_corr_z_4_rangea: Default::default(),
            tp_uwr_lng_corr_z_4_rangeb: Default::default(),
            tp_uwr_lng_corr_z_5_rangea: Default::default(),
            tp_uwr_lng_corr_z_5_rangeb: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_zone_config_t {
    fn default() -> Self {
        Self {
            max_zones: Default::default(),
            active_zones: Default::default(),
            multizone_hist_cfg: Default::default(),
            user_zones: Default::default(),
            bin_config: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_optical_centre_t {
    fn default() -> Self {
        Self {
            x_centre: Default::default(),
            y_centre: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_user_zone_t {
    fn default() -> Self {
        Self {
            x_centre: Default::default(),
            y_centre: Default::default(),
            width: Default::default(),
            height: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_gain_calibration_data_t {
    fn default() -> Self {
        Self {
            standard_ranging_gain_factor: Default::default(),
            histogram_ranging_gain_factor: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_dmax_calibration_data_t {
    fn default() -> Self {
        Self {
            ref__actual_effective_spads: Default::default(),
            ref__peak_signal_count_rate_mcps: Default::default(),
            ref__distance_mm: Default::default(),
            ref_reflectance_pc: Default::default(),
            coverglass_transmission: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_additional_offset_cal_data_t {
    fn default() -> Self {
        Self {
            result__mm_inner_actual_effective_spads: Default::default(),
            result__mm_outer_actual_effective_spads: Default::default(),
            result__mm_inner_peak_signal_count_rtn_mcps: Default::default(),
            result__mm_outer_peak_signal_count_rtn_mcps: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_cal_peak_rate_map_t {
    fn default() -> Self {
        Self {
            cal_distance_mm: Default::default(),
            cal_reflectance_pc: Default::default(),
            max_samples: Default::default(),
            width: Default::default(),
            height: Default::default(),
            peak_rate_mcps: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_customer_nvm_managed_t {
    fn default() -> Self {
        Self {
            global_config__spad_enables_ref_0: Default::default(),
            global_config__spad_enables_ref_1: Default::default(),
            global_config__spad_enables_ref_2: Default::default(),
            global_config__spad_enables_ref_3: Default::default(),
            global_config__spad_enables_ref_4: Default::default(),
            global_config__spad_enables_ref_5: Default::default(),
            global_config__ref_en_start_select: Default::default(),
            ref_spad_man__num_requested_ref_spads: Default::default(),
            ref_spad_man__ref_location: Default::default(),
            algo__crosstalk_compensation_plane_offset_kcps: Default::default(),
            algo__crosstalk_compensation_x_plane_gradient_kcps: Default::default(),
            algo__crosstalk_compensation_y_plane_gradient_kcps: Default::default(),
            ref_spad_char__total_rate_target_mcps: Default::default(),
            algo__part_to_part_range_offset_mm: Default::default(),
            mm_config__inner_offset_mm: Default::default(),
            mm_config__outer_offset_mm: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_GPIO_interrupt_config_t {
    fn default() -> Self {
        Self {
            intr_mode_distance: Default::default(),
            intr_mode_rate: Default::default(),
            intr_new_measure_ready: Default::default(),
            intr_no_target: Default::default(),
            intr_combined_mode: Default::default(),
            threshold_distance_high: Default::default(),
            threshold_distance_low: Default::default(),
            threshold_rate_high: Default::default(),
            threshold_rate_low: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_ll_driver_state_t {
    fn default() -> Self {
        Self {
            cfg_device_state: Default::default(),
            cfg_stream_count: Default::default(),
            cfg_internal_stream_count: Default::default(),
            cfg_internal_stream_count_val: Default::default(),
            cfg_gph_id: Default::default(),
            cfg_timing_status: Default::default(),
            cfg_zone_id: Default::default(),
            rd_device_state: Default::default(),
            rd_stream_count: Default::default(),
            rd_internal_stream_count: Default::default(),
            rd_internal_stream_count_val: Default::default(),
            rd_gph_id: Default::default(),
            rd_timing_status: Default::default(),
            rd_zone_id: Default::default(),
        }
    }
}
impl Default for bindings::VL53LX_ll_version_t {
    fn default() -> Self {
        Self {
            ll_revision: Default::default(),
            ll_major: Default::default(),
            ll_minor: Default::default(),
            ll_build: Default::default(),
        }
    }
}
