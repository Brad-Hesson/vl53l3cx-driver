
/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/******************************************************************************
 * Copyright (c) 2020, STMicroelectronics - All Rights Reserved

 This file is part of VL53LX and is dual licensed,
 either GPL-2.0+
 or 'BSD 3-clause "New" or "Revised" License' , at your option.
 ******************************************************************************
 */




#ifndef _VL53LX_LL_DEVICE_H_
#define _VL53LX_LL_DEVICE_H_

#include "vl53lx_platform_user_config.h"
#include "vl53lx_types.h"

#define   VL53LX_I2C                      0x01
#define   VL53LX_SPI                      0x00





typedef uint8_t VL53LX_WaitMethod;

#define VL53LX_WAIT_METHOD_BLOCKING               ((VL53LX_WaitMethod)  0)
#define VL53LX_WAIT_METHOD_NON_BLOCKING           ((VL53LX_WaitMethod)  1)




typedef uint8_t VL53LX_DeviceState;

#define VL53LX_DEVICESTATE_POWERDOWN              ((VL53LX_DeviceState)  0)
#define VL53LX_DEVICESTATE_HW_STANDBY             ((VL53LX_DeviceState)  1)
#define VL53LX_DEVICESTATE_FW_COLDBOOT            ((VL53LX_DeviceState)  2)
#define VL53LX_DEVICESTATE_SW_STANDBY             ((VL53LX_DeviceState)  3)
#define VL53LX_DEVICESTATE_RANGING_DSS_AUTO       ((VL53LX_DeviceState)  4)
#define VL53LX_DEVICESTATE_RANGING_DSS_MANUAL     ((VL53LX_DeviceState)  5)
#define VL53LX_DEVICESTATE_RANGING_WAIT_GPH_SYNC  ((VL53LX_DeviceState)  6)
#define VL53LX_DEVICESTATE_RANGING_GATHER_DATA    ((VL53LX_DeviceState)  7)
#define VL53LX_DEVICESTATE_RANGING_OUTPUT_DATA    ((VL53LX_DeviceState)  8)

#define VL53LX_DEVICESTATE_UNKNOWN               ((VL53LX_DeviceState) 98)
#define VL53LX_DEVICESTATE_ERROR                 ((VL53LX_DeviceState) 99)



typedef uint8_t VL53LX_DeviceZonePreset;


typedef uint8_t VL53LX_DevicePresetModes;

#define VL53LX_DEVICEPRESETMODE_NONE                            \
	((VL53LX_DevicePresetModes)  0)
#define VL53LX_DEVICEPRESETMODE_STANDARD_RANGING                \
	((VL53LX_DevicePresetModes)  1)
#define VL53LX_DEVICEPRESETMODE_HISTOGRAM_LONG_RANGE            \
	((VL53LX_DevicePresetModes) 27)
#define VL53LX_DEVICEPRESETMODE_HISTOGRAM_MEDIUM_RANGE          \
	((VL53LX_DevicePresetModes) 30)
#define VL53LX_DEVICEPRESETMODE_HISTOGRAM_SHORT_RANGE           \
	((VL53LX_DevicePresetModes) 33)





typedef uint8_t VL53LX_DeviceMeasurementModes;

#define VL53LX_DEVICEMEASUREMENTMODE_STOP          \
	((VL53LX_DeviceMeasurementModes)  0x00)
#define VL53LX_DEVICEMEASUREMENTMODE_SINGLESHOT     \
	((VL53LX_DeviceMeasurementModes)  0x10)
#define VL53LX_DEVICEMEASUREMENTMODE_BACKTOBACK      \
	((VL53LX_DeviceMeasurementModes)  0x20)
#define VL53LX_DEVICEMEASUREMENTMODE_TIMED          \
	((VL53LX_DeviceMeasurementModes)  0x40)
#define VL53LX_DEVICEMEASUREMENTMODE_ABORT          \
	((VL53LX_DeviceMeasurementModes)  0x80)





typedef uint8_t VL53LX_OffsetCalibrationMode;

#define VL53LX_OFFSETCALIBRATIONMODE__NONE                \
	((VL53LX_OffsetCalibrationMode)  0)
#define VL53LX_OFFSETCALIBRATIONMODE__MM1_MM2__STANDARD    \
	((VL53LX_OffsetCalibrationMode)  1)
#define VL53LX_OFFSETCALIBRATIONMODE__MM1_MM2__HISTOGRAM    \
	((VL53LX_OffsetCalibrationMode)  2)
#define VL53LX_OFFSETCALIBRATIONMODE__MM1_MM2__STANDARD_PRE_RANGE_ONLY \
	((VL53LX_OffsetCalibrationMode)  3)
#define VL53LX_OFFSETCALIBRATIONMODE__MM1_MM2__HISTOGRAM_PRE_RANGE_ONLY \
	((VL53LX_OffsetCalibrationMode)  4)





typedef uint8_t VL53LX_OffsetCorrectionMode;

#define VL53LX_OFFSETCORRECTIONMODE__NONE             \
	((VL53LX_OffsetCorrectionMode)  0)
#define VL53LX_OFFSETCORRECTIONMODE__MM1_MM2_OFFSETS  \
	((VL53LX_OffsetCorrectionMode)  1)
#define VL53LX_OFFSETCORRECTIONMODE__PER_VCSEL_OFFSETS  \
	((VL53LX_OffsetCorrectionMode)  3)





typedef uint8_t VL53LX_DeviceDmaxMode;

#define VL53LX_DEVICEDMAXMODE__NONE                 \
	((VL53LX_DeviceDmaxMode)  0)
#define VL53LX_DEVICEDMAXMODE__FMT_CAL_DATA          \
	((VL53LX_DeviceDmaxMode)  1)
#define VL53LX_DEVICEDMAXMODE__CUST_CAL_DATA         \
	((VL53LX_DeviceDmaxMode)  2)





typedef uint8_t VL53LX_DeviceSequenceConfig;

#define VL53LX_DEVICESEQUENCECONFIG_VHV		\
	((VL53LX_DeviceSequenceConfig) 0)
#define VL53LX_DEVICESEQUENCECONFIG_PHASECAL     \
	((VL53LX_DeviceSequenceConfig) 1)
#define VL53LX_DEVICESEQUENCECONFIG_REFERENCE_PHASE \
	((VL53LX_DeviceSequenceConfig) 2)
#define VL53LX_DEVICESEQUENCECONFIG_DSS1           \
	((VL53LX_DeviceSequenceConfig) 3)
#define VL53LX_DEVICESEQUENCECONFIG_DSS2           \
	((VL53LX_DeviceSequenceConfig) 4)
#define VL53LX_DEVICESEQUENCECONFIG_MM1            \
	((VL53LX_DeviceSequenceConfig) 5)
#define VL53LX_DEVICESEQUENCECONFIG_MM2            \
	((VL53LX_DeviceSequenceConfig) 6)
#define VL53LX_DEVICESEQUENCECONFIG_RANGE          \
	((VL53LX_DeviceSequenceConfig) 7)





typedef uint8_t VL53LX_DeviceInterruptPolarity;

#define VL53LX_DEVICEINTERRUPTPOLARITY_ACTIVE_HIGH        \
	((VL53LX_DeviceInterruptPolarity)  0x00)
#define VL53LX_DEVICEINTERRUPTPOLARITY_ACTIVE_LOW         \
	((VL53LX_DeviceInterruptPolarity)  0x10)
#define VL53LX_DEVICEINTERRUPTPOLARITY_BIT_MASK           \
	((VL53LX_DeviceInterruptPolarity)  0x10)
#define VL53LX_DEVICEINTERRUPTPOLARITY_CLEAR_MASK         \
	((VL53LX_DeviceInterruptPolarity)  0xEF)





typedef uint8_t VL53LX_DeviceGpioMode;

#define VL53LX_DEVICEGPIOMODE_OUTPUT_CONSTANT_ZERO                \
	((VL53LX_DeviceGpioMode)  0x00)
#define VL53LX_DEVICEGPIOMODE_OUTPUT_RANGE_AND_ERROR_INTERRUPTS    \
	((VL53LX_DeviceGpioMode)  0x01)
#define VL53LX_DEVICEGPIOMODE_OUTPUT_TIMIER_INTERRUPTS             \
	((VL53LX_DeviceGpioMode)  0x02)
#define VL53LX_DEVICEGPIOMODE_OUTPUT_RANGE_MODE_INTERRUPT_STATUS  \
	((VL53LX_DeviceGpioMode)  0x03)
#define VL53LX_DEVICEGPIOMODE_OUTPUT_SLOW_OSCILLATOR_CLOCK        \
	((VL53LX_DeviceGpioMode)  0x04)
#define VL53LX_DEVICEGPIOMODE_BIT_MASK                           \
	((VL53LX_DeviceGpioMode)  0x0F)
#define VL53LX_DEVICEGPIOMODE_CLEAR_MASK                        \
	((VL53LX_DeviceGpioMode)  0xF0)





typedef uint8_t VL53LX_DeviceError;

#define VL53LX_DEVICEERROR_NOUPDATE                   \
	((VL53LX_DeviceError) 0)

#define VL53LX_DEVICEERROR_VCSELCONTINUITYTESTFAILURE \
	((VL53LX_DeviceError) 1)
#define VL53LX_DEVICEERROR_VCSELWATCHDOGTESTFAILURE   \
	((VL53LX_DeviceError) 2)
#define VL53LX_DEVICEERROR_NOVHVVALUEFOUND            \
	((VL53LX_DeviceError) 3)
#define VL53LX_DEVICEERROR_MSRCNOTARGET               \
	((VL53LX_DeviceError) 4)
#define VL53LX_DEVICEERROR_RANGEPHASECHECK            \
	((VL53LX_DeviceError) 5)
#define VL53LX_DEVICEERROR_SIGMATHRESHOLDCHECK        \
	((VL53LX_DeviceError) 6)
#define VL53LX_DEVICEERROR_PHASECONSISTENCY           \
	((VL53LX_DeviceError) 7)
#define VL53LX_DEVICEERROR_MINCLIP                    \
	((VL53LX_DeviceError) 8)
#define VL53LX_DEVICEERROR_RANGECOMPLETE               \
	((VL53LX_DeviceError) 9)
#define VL53LX_DEVICEERROR_ALGOUNDERFLOW               \
	((VL53LX_DeviceError) 10)
#define VL53LX_DEVICEERROR_ALGOOVERFLOW                \
	((VL53LX_DeviceError) 11)
#define VL53LX_DEVICEERROR_RANGEIGNORETHRESHOLD       \
	((VL53LX_DeviceError) 12)
#define VL53LX_DEVICEERROR_USERROICLIP                \
	((VL53LX_DeviceError) 13)
#define VL53LX_DEVICEERROR_REFSPADCHARNOTENOUGHDPADS   \
	((VL53LX_DeviceError) 14)
#define VL53LX_DEVICEERROR_REFSPADCHARMORETHANTARGET  \
	((VL53LX_DeviceError) 15)
#define VL53LX_DEVICEERROR_REFSPADCHARLESSTHANTARGET  \
	((VL53LX_DeviceError) 16)
#define VL53LX_DEVICEERROR_MULTCLIPFAIL                \
	((VL53LX_DeviceError) 17)
#define VL53LX_DEVICEERROR_GPHSTREAMCOUNT0READY        \
	((VL53LX_DeviceError) 18)
#define VL53LX_DEVICEERROR_RANGECOMPLETE_NO_WRAP_CHECK \
	((VL53LX_DeviceError) 19)
#define VL53LX_DEVICEERROR_EVENTCONSISTENCY           \
	((VL53LX_DeviceError) 20)
#define VL53LX_DEVICEERROR_MINSIGNALEVENTCHECK        \
	((VL53LX_DeviceError) 21)
#define VL53LX_DEVICEERROR_RANGECOMPLETE_MERGED_PULSE \
	((VL53LX_DeviceError) 22)


#define VL53LX_DEVICEERROR_PREV_RANGE_NO_TARGETS      \
	((VL53LX_DeviceError) 23)





typedef uint8_t VL53LX_DeviceReportStatus;

#define VL53LX_DEVICEREPORTSTATUS_NOUPDATE                 \
	((VL53LX_DeviceReportStatus) 0)

#define VL53LX_DEVICEREPORTSTATUS_ROI_SETUP               \
	((VL53LX_DeviceReportStatus)  1)
#define VL53LX_DEVICEREPORTSTATUS_VHV                     \
	((VL53LX_DeviceReportStatus)  2)
#define VL53LX_DEVICEREPORTSTATUS_PHASECAL                \
	((VL53LX_DeviceReportStatus)  3)
#define VL53LX_DEVICEREPORTSTATUS_REFERENCE_PHASE         \
	((VL53LX_DeviceReportStatus)  4)
#define VL53LX_DEVICEREPORTSTATUS_DSS1                    \
	((VL53LX_DeviceReportStatus)  5)
#define VL53LX_DEVICEREPORTSTATUS_DSS2                    \
	((VL53LX_DeviceReportStatus)  6)
#define VL53LX_DEVICEREPORTSTATUS_MM1                     \
	((VL53LX_DeviceReportStatus)  7)
#define VL53LX_DEVICEREPORTSTATUS_MM2                     \
	((VL53LX_DeviceReportStatus)  8)
#define VL53LX_DEVICEREPORTSTATUS_RANGE                   \
	((VL53LX_DeviceReportStatus)  9)
#define VL53LX_DEVICEREPORTSTATUS_HISTOGRAM               \
	((VL53LX_DeviceReportStatus) 10)





typedef uint8_t VL53LX_DeviceDssMode;

#define VL53LX_DEVICEDSSMODE__DISABLED \
	((VL53LX_DeviceDssMode) 0)
#define VL53LX_DEVICEDSSMODE__TARGET_RATE \
	((VL53LX_DeviceDssMode) 1)
#define VL53LX_DEVICEDSSMODE__REQUESTED_EFFFECTIVE_SPADS \
	((VL53LX_DeviceDssMode) 2)
#define VL53LX_DEVICEDSSMODE__BLOCK_SELECT \
	((VL53LX_DeviceDssMode) 3)






typedef uint8_t VL53LX_HistAlgoSelect;

#define VL53LX_HIST_ALGO_SELECT__PW_HIST_GEN1 \
	((VL53LX_HistAlgoSelect) 1)
#define VL53LX_HIST_ALGO_SELECT__PW_HIST_GEN2 \
	((VL53LX_HistAlgoSelect) 2)
#define VL53LX_HIST_ALGO_SELECT__PW_HIST_GEN3 \
	((VL53LX_HistAlgoSelect) 3)
#define VL53LX_HIST_ALGO_SELECT__PW_HIST_GEN4 \
	((VL53LX_HistAlgoSelect) 4)






typedef uint8_t VL53LX_HistTargetOrder;

#define VL53LX_HIST_TARGET_ORDER__INCREASING_DISTANCE \
	((VL53LX_HistTargetOrder) 1)
#define VL53LX_HIST_TARGET_ORDER__STRONGEST_FIRST \
	((VL53LX_HistTargetOrder) 2)






typedef uint8_t VL53LX_HistAmbEstMethod;

#define VL53LX_HIST_AMB_EST_METHOD__AMBIENT_BINS \
	((VL53LX_HistAmbEstMethod) 1)
#define VL53LX_HIST_AMB_EST_METHOD__THRESHOLDED_BINS  \
	((VL53LX_HistAmbEstMethod) 2)






typedef uint8_t VL53LX_HistXtalkCompEnable;

#define VL53LX_HIST_XTALK_COMP__DIS \
	((VL53LX_HistXtalkCompEnable) 0)
#define VL53LX_HIST_XTALK_COMP__EN \
	((VL53LX_HistXtalkCompEnable) 1)




typedef uint8_t VL53LX_DeviceConfigLevel;

#define VL53LX_DEVICECONFIGLEVEL_SYSTEM_CONTROL  \
	((VL53LX_DeviceConfigLevel)  0)

#define VL53LX_DEVICECONFIGLEVEL_DYNAMIC_ONWARDS \
	((VL53LX_DeviceConfigLevel)  1)

#define VL53LX_DEVICECONFIGLEVEL_TIMING_ONWARDS \
	((VL53LX_DeviceConfigLevel)  2)

#define VL53LX_DEVICECONFIGLEVEL_GENERAL_ONWARDS \
	((VL53LX_DeviceConfigLevel)  3)

#define VL53LX_DEVICECONFIGLEVEL_STATIC_ONWARDS  \
	((VL53LX_DeviceConfigLevel)  4)

#define VL53LX_DEVICECONFIGLEVEL_CUSTOMER_ONWARDS  \
	((VL53LX_DeviceConfigLevel)  5)

#define VL53LX_DEVICECONFIGLEVEL_FULL  \
	((VL53LX_DeviceConfigLevel)  6)






typedef uint8_t VL53LX_DeviceResultsLevel;

#define VL53LX_DEVICERESULTSLEVEL_SYSTEM_RESULTS  \
	((VL53LX_DeviceResultsLevel)  0)

#define VL53LX_DEVICERESULTSLEVEL_UPTO_CORE  \
	((VL53LX_DeviceResultsLevel)  1)

#define VL53LX_DEVICERESULTSLEVEL_FULL  \
	((VL53LX_DeviceResultsLevel)  2)







typedef uint8_t VL53LX_DeviceTestMode;

#define VL53LX_DEVICETESTMODE_NONE \
	((VL53LX_DeviceTestMode) 0x00)

#define VL53LX_DEVICETESTMODE_NVM_ZERO \
	((VL53LX_DeviceTestMode) 0x01)

#define VL53LX_DEVICETESTMODE_NVM_COPY \
	((VL53LX_DeviceTestMode) 0x02)

#define VL53LX_DEVICETESTMODE_PATCH \
	((VL53LX_DeviceTestMode) 0x03)

#define VL53LX_DEVICETESTMODE_DCR \
	((VL53LX_DeviceTestMode) 0x04)

#define VL53LX_DEVICETESTMODE_LCR_VCSEL_OFF \
	((VL53LX_DeviceTestMode) 0x05)

#define VL53LX_DEVICETESTMODE_LCR_VCSEL_ON \
	((VL53LX_DeviceTestMode) 0x06)

#define VL53LX_DEVICETESTMODE_SPOT_CENTRE_LOCATE \
	((VL53LX_DeviceTestMode) 0x07)

#define VL53LX_DEVICETESTMODE_REF_SPAD_CHAR_WITH_PRE_VHV \
	((VL53LX_DeviceTestMode) 0x08)

#define VL53LX_DEVICETESTMODE_REF_SPAD_CHAR_ONLY \
	((VL53LX_DeviceTestMode) 0x09)







typedef uint8_t VL53LX_DeviceSscArray;

#define VL53LX_DEVICESSCARRAY_RTN ((VL53LX_DeviceSscArray) 0x00)

#define VL53LX_DEVICETESTMODE_REF ((VL53LX_DeviceSscArray) 0x01)







#define VL53LX_RETURN_ARRAY_ONLY                   0x01

#define VL53LX_REFERENCE_ARRAY_ONLY                0x10

#define VL53LX_BOTH_RETURN_AND_REFERENCE_ARRAYS    0x11

#define VL53LX_NEITHER_RETURN_AND_REFERENCE_ARRAYS 0x00






#define VL53LX_DEVICEINTERRUPTLEVEL_ACTIVE_HIGH               0x00

#define VL53LX_DEVICEINTERRUPTLEVEL_ACTIVE_LOW                0x10

#define VL53LX_DEVICEINTERRUPTLEVEL_ACTIVE_MASK               0x10






#define VL53LX_POLLING_DELAY_US                     1000

#define VL53LX_SOFTWARE_RESET_DURATION_US            100

#define VL53LX_FIRMWARE_BOOT_TIME_US                1200

#define VL53LX_ENABLE_POWERFORCE_SETTLING_TIME_US    250

#define VL53LX_SPAD_ARRAY_WIDTH                       16

#define VL53LX_SPAD_ARRAY_HEIGHT                      16

#define VL53LX_NVM_SIZE_IN_BYTES                     512

#define VL53LX_NO_OF_SPAD_ENABLES                    256

#define VL53LX_RTN_SPAD_BUFFER_SIZE                   32

#define VL53LX_REF_SPAD_BUFFER_SIZE                    6

#define VL53LX_AMBIENT_WINDOW_VCSEL_PERIODS          256

#define VL53LX_RANGING_WINDOW_VCSEL_PERIODS         2048

#define VL53LX_MACRO_PERIOD_VCSEL_PERIODS \
	(VL53LX_AMBIENT_WINDOW_VCSEL_PERIODS + \
		VL53LX_RANGING_WINDOW_VCSEL_PERIODS)

#define VL53LX_MAX_ALLOWED_PHASE                    0xFFFF


#define VL53LX_RTN_SPAD_UNITY_TRANSMISSION      0x0100

#define VL53LX_RTN_SPAD_APERTURE_TRANSMISSION   0x0038


#define VL53LX_SPAD_TOTAL_COUNT_MAX                 ((0x01 << 29) - 1)

#define VL53LX_SPAD_TOTAL_COUNT_RES_THRES            (0x01 << 24)

#define VL53LX_COUNT_RATE_INTERNAL_MAX              ((0x01 << 24) - 1)

#define VL53LX_SPEED_OF_LIGHT_IN_AIR                299704

#define VL53LX_SPEED_OF_LIGHT_IN_AIR_DIV_8          (299704 >> 3)








typedef uint8_t VL53LX_ZoneConfig_BinConfig_select;

#define VL53LX_ZONECONFIG_BINCONFIG__LOWAMB \
	((VL53LX_ZoneConfig_BinConfig_select) 1)
#define VL53LX_ZONECONFIG_BINCONFIG__MIDAMB \
	((VL53LX_ZoneConfig_BinConfig_select) 2)
#define VL53LX_ZONECONFIG_BINCONFIG__HIGHAMB \
	((VL53LX_ZoneConfig_BinConfig_select) 3)





typedef uint8_t VL53LX_GPIO_Interrupt_Mode;

#define VL53LX_GPIOINTMODE_LEVEL_LOW \
	((VL53LX_GPIO_Interrupt_Mode) 0)

#define VL53LX_GPIOINTMODE_LEVEL_HIGH \
	((VL53LX_GPIO_Interrupt_Mode) 1)

#define VL53LX_GPIOINTMODE_OUT_OF_WINDOW \
	((VL53LX_GPIO_Interrupt_Mode) 2)

#define VL53LX_GPIOINTMODE_IN_WINDOW \
	((VL53LX_GPIO_Interrupt_Mode) 3)






typedef uint16_t VL53LX_TuningParms;

#define VL53LX_TUNINGPARMS_LLD_PUBLIC_MIN_ADDRESS \
	((VL53LX_TuningParms) VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS)
#define VL53LX_TUNINGPARMS_LLD_PUBLIC_MAX_ADDRESS \
	((VL53LX_TuningParms) VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_5_RANGEB)

#define VL53LX_TUNINGPARMS_LLD_PRIVATE_MIN_ADDRESS \
	((VL53LX_TuningParms) VL53LX_TUNINGPARM_PRIVATE_PAGE_BASE_ADDRESS)
#define VL53LX_TUNINGPARMS_LLD_PRIVATE_MAX_ADDRESS \
	((VL53LX_TuningParms) VL53LX_TUNINGPARMS_LLD_PRIVATE_MIN_ADDRESS)

#define VL53LX_TUNINGPARM_VERSION \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 0))
#define VL53LX_TUNINGPARM_KEY_TABLE_VERSION \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 1))
#define VL53LX_TUNINGPARM_LLD_VERSION \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 2))
#define VL53LX_TUNINGPARM_HIST_ALGO_SELECT \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 3))
#define VL53LX_TUNINGPARM_HIST_TARGET_ORDER \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 4))
#define VL53LX_TUNINGPARM_HIST_FILTER_WOI_0 \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 5))
#define VL53LX_TUNINGPARM_HIST_FILTER_WOI_1 \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 6))
#define VL53LX_TUNINGPARM_HIST_AMB_EST_METHOD \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 7))
#define VL53LX_TUNINGPARM_HIST_AMB_THRESH_SIGMA_0 \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 8))
#define VL53LX_TUNINGPARM_HIST_AMB_THRESH_SIGMA_1 \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 9))
#define VL53LX_TUNINGPARM_HIST_MIN_AMB_THRESH_EVENTS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 10))
#define VL53LX_TUNINGPARM_HIST_AMB_EVENTS_SCALER \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 11))
#define VL53LX_TUNINGPARM_HIST_NOISE_THRESHOLD \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 12))
#define VL53LX_TUNINGPARM_HIST_SIGNAL_TOTAL_EVENTS_LIMIT \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 13))
#define VL53LX_TUNINGPARM_HIST_SIGMA_EST_REF_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 14))
#define VL53LX_TUNINGPARM_HIST_SIGMA_THRESH_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 15))
#define VL53LX_TUNINGPARM_HIST_GAIN_FACTOR \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 16))
#define VL53LX_TUNINGPARM_CONSISTENCY_HIST_PHASE_TOLERANCE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 17))
#define VL53LX_TUNINGPARM_CONSISTENCY_HIST_MIN_MAX_TOLERANCE_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 18))
#define VL53LX_TUNINGPARM_CONSISTENCY_HIST_EVENT_SIGMA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 19))
#define VL53LX_TUNINGPARM_CONSISTENCY_HIST_EVENT_SIGMA_MIN_SPAD_LIMIT \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 20))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_RTN_HISTO_LONG_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 21))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_RTN_HISTO_MED_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 22))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_RTN_HISTO_SHORT_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 23))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_REF_HISTO_LONG_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 24))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_REF_HISTO_MED_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 25))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_REF_HISTO_SHORT_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 26))
#define VL53LX_TUNINGPARM_XTALK_DETECT_MIN_VALID_RANGE_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 27))
#define VL53LX_TUNINGPARM_XTALK_DETECT_MAX_VALID_RANGE_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 28))
#define VL53LX_TUNINGPARM_XTALK_DETECT_MAX_SIGMA_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 29))
#define VL53LX_TUNINGPARM_XTALK_DETECT_MIN_MAX_TOLERANCE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 30))
#define VL53LX_TUNINGPARM_XTALK_DETECT_MAX_VALID_RATE_KCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 31))
#define VL53LX_TUNINGPARM_XTALK_DETECT_EVENT_SIGMA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 32))
#define VL53LX_TUNINGPARM_HIST_XTALK_MARGIN_KCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 33))
#define VL53LX_TUNINGPARM_CONSISTENCY_LITE_PHASE_TOLERANCE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 34))
#define VL53LX_TUNINGPARM_PHASECAL_TARGET \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 35))
#define VL53LX_TUNINGPARM_LITE_CAL_REPEAT_RATE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 36))
#define VL53LX_TUNINGPARM_LITE_RANGING_GAIN_FACTOR \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 37))
#define VL53LX_TUNINGPARM_LITE_MIN_CLIP_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 38))
#define VL53LX_TUNINGPARM_LITE_LONG_SIGMA_THRESH_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 39))
#define VL53LX_TUNINGPARM_LITE_MED_SIGMA_THRESH_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 40))
#define VL53LX_TUNINGPARM_LITE_SHORT_SIGMA_THRESH_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 41))
#define VL53LX_TUNINGPARM_LITE_LONG_MIN_COUNT_RATE_RTN_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 42))
#define VL53LX_TUNINGPARM_LITE_MED_MIN_COUNT_RATE_RTN_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 43))
#define VL53LX_TUNINGPARM_LITE_SHORT_MIN_COUNT_RATE_RTN_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 44))
#define VL53LX_TUNINGPARM_LITE_SIGMA_EST_PULSE_WIDTH \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 45))
#define VL53LX_TUNINGPARM_LITE_SIGMA_EST_AMB_WIDTH_NS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 46))
#define VL53LX_TUNINGPARM_LITE_SIGMA_REF_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 47))
#define VL53LX_TUNINGPARM_LITE_RIT_MULT \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 48))
#define VL53LX_TUNINGPARM_LITE_SEED_CONFIG \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 49))
#define VL53LX_TUNINGPARM_LITE_QUANTIFIER \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 50))
#define VL53LX_TUNINGPARM_LITE_FIRST_ORDER_SELECT \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 51))
#define VL53LX_TUNINGPARM_LITE_XTALK_MARGIN_KCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 52))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_RTN_LITE_LONG_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 53))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_RTN_LITE_MED_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 54))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_RTN_LITE_SHORT_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 55))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_REF_LITE_LONG_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 56))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_REF_LITE_MED_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 57))
#define VL53LX_TUNINGPARM_INITIAL_PHASE_REF_LITE_SHORT_RANGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 58))
#define VL53LX_TUNINGPARM_TIMED_SEED_CONFIG \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 59))
#define VL53LX_TUNINGPARM_DMAX_CFG_SIGNAL_THRESH_SIGMA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 60))
#define VL53LX_TUNINGPARM_DMAX_CFG_REFLECTANCE_ARRAY_0 \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 61))
#define VL53LX_TUNINGPARM_DMAX_CFG_REFLECTANCE_ARRAY_1 \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 62))
#define VL53LX_TUNINGPARM_DMAX_CFG_REFLECTANCE_ARRAY_2 \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 63))
#define VL53LX_TUNINGPARM_DMAX_CFG_REFLECTANCE_ARRAY_3 \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 64))
#define VL53LX_TUNINGPARM_DMAX_CFG_REFLECTANCE_ARRAY_4 \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 65))
#define VL53LX_TUNINGPARM_VHV_LOOPBOUND \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 66))
#define VL53LX_TUNINGPARM_REFSPADCHAR_DEVICE_TEST_MODE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 67))
#define VL53LX_TUNINGPARM_REFSPADCHAR_VCSEL_PERIOD \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 68))
#define VL53LX_TUNINGPARM_REFSPADCHAR_PHASECAL_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 69))
#define VL53LX_TUNINGPARM_REFSPADCHAR_TARGET_COUNT_RATE_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 70))
#define VL53LX_TUNINGPARM_REFSPADCHAR_MIN_COUNTRATE_LIMIT_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 71))
#define VL53LX_TUNINGPARM_REFSPADCHAR_MAX_COUNTRATE_LIMIT_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 72))
#define VL53LX_TUNINGPARM_XTALK_EXTRACT_NUM_OF_SAMPLES \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 73))
#define VL53LX_TUNINGPARM_XTALK_EXTRACT_MIN_FILTER_THRESH_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 74))
#define VL53LX_TUNINGPARM_XTALK_EXTRACT_MAX_FILTER_THRESH_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 75))
#define VL53LX_TUNINGPARM_XTALK_EXTRACT_DSS_RATE_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 76))
#define VL53LX_TUNINGPARM_XTALK_EXTRACT_PHASECAL_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 77))
#define VL53LX_TUNINGPARM_XTALK_EXTRACT_MAX_VALID_RATE_KCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 78))
#define VL53LX_TUNINGPARM_XTALK_EXTRACT_SIGMA_THRESHOLD_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 79))
#define VL53LX_TUNINGPARM_XTALK_EXTRACT_DSS_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 80))
#define VL53LX_TUNINGPARM_XTALK_EXTRACT_BIN_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 81))
#define VL53LX_TUNINGPARM_OFFSET_CAL_DSS_RATE_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 82))
#define VL53LX_TUNINGPARM_OFFSET_CAL_PHASECAL_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 83))
#define VL53LX_TUNINGPARM_OFFSET_CAL_MM_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 84))
#define VL53LX_TUNINGPARM_OFFSET_CAL_RANGE_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 85))
#define VL53LX_TUNINGPARM_OFFSET_CAL_PRE_SAMPLES \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 86))
#define VL53LX_TUNINGPARM_OFFSET_CAL_MM1_SAMPLES \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 87))
#define VL53LX_TUNINGPARM_OFFSET_CAL_MM2_SAMPLES \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 88))
#define VL53LX_TUNINGPARM_ZONE_CAL_DSS_RATE_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 89))
#define VL53LX_TUNINGPARM_ZONE_CAL_PHASECAL_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 90))
#define VL53LX_TUNINGPARM_ZONE_CAL_DSS_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 91))
#define VL53LX_TUNINGPARM_ZONE_CAL_PHASECAL_NUM_SAMPLES \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 92))
#define VL53LX_TUNINGPARM_ZONE_CAL_RANGE_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 93))
#define VL53LX_TUNINGPARM_ZONE_CAL_ZONE_NUM_SAMPLES \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 94))
#define VL53LX_TUNINGPARM_SPADMAP_VCSEL_PERIOD \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 95))
#define VL53LX_TUNINGPARM_SPADMAP_VCSEL_START \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 96))
#define VL53LX_TUNINGPARM_SPADMAP_RATE_LIMIT_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 97))
#define VL53LX_TUNINGPARM_LITE_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 98))
#define VL53LX_TUNINGPARM_RANGING_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 99))
#define VL53LX_TUNINGPARM_MZ_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 100))
#define VL53LX_TUNINGPARM_TIMED_DSS_CONFIG_TARGET_TOTAL_RATE_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 101))
#define VL53LX_TUNINGPARM_LITE_PHASECAL_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 102))
#define VL53LX_TUNINGPARM_RANGING_LONG_PHASECAL_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 103))
#define VL53LX_TUNINGPARM_RANGING_MED_PHASECAL_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 104))
#define VL53LX_TUNINGPARM_RANGING_SHORT_PHASECAL_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 105))
#define VL53LX_TUNINGPARM_MZ_LONG_PHASECAL_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 106))
#define VL53LX_TUNINGPARM_MZ_MED_PHASECAL_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 107))
#define VL53LX_TUNINGPARM_MZ_SHORT_PHASECAL_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 108))
#define VL53LX_TUNINGPARM_TIMED_PHASECAL_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 109))
#define VL53LX_TUNINGPARM_LITE_MM_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 110))
#define VL53LX_TUNINGPARM_RANGING_MM_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 111))
#define VL53LX_TUNINGPARM_MZ_MM_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 112))
#define VL53LX_TUNINGPARM_TIMED_MM_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 113))
#define VL53LX_TUNINGPARM_LITE_RANGE_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 114))
#define VL53LX_TUNINGPARM_RANGING_RANGE_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 115))
#define VL53LX_TUNINGPARM_MZ_RANGE_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 116))
#define VL53LX_TUNINGPARM_TIMED_RANGE_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 117))
#define VL53LX_TUNINGPARM_DYNXTALK_SMUDGE_MARGIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 118))
#define VL53LX_TUNINGPARM_DYNXTALK_NOISE_MARGIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 119))
#define VL53LX_TUNINGPARM_DYNXTALK_XTALK_OFFSET_LIMIT \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 120))
#define VL53LX_TUNINGPARM_DYNXTALK_XTALK_OFFSET_LIMIT_HI \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 121))
#define VL53LX_TUNINGPARM_DYNXTALK_SAMPLE_LIMIT \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 122))
#define VL53LX_TUNINGPARM_DYNXTALK_SINGLE_XTALK_DELTA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 123))
#define VL53LX_TUNINGPARM_DYNXTALK_AVERAGED_XTALK_DELTA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 124))
#define VL53LX_TUNINGPARM_DYNXTALK_CLIP_LIMIT \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 125))
#define VL53LX_TUNINGPARM_DYNXTALK_SCALER_CALC_METHOD \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 126))
#define VL53LX_TUNINGPARM_DYNXTALK_XGRADIENT_SCALER \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 127))
#define VL53LX_TUNINGPARM_DYNXTALK_YGRADIENT_SCALER \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 128))
#define VL53LX_TUNINGPARM_DYNXTALK_USER_SCALER_SET \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 129))
#define VL53LX_TUNINGPARM_DYNXTALK_SMUDGE_COR_SINGLE_APPLY \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 130))
#define VL53LX_TUNINGPARM_DYNXTALK_XTALK_AMB_THRESHOLD \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 131))
#define VL53LX_TUNINGPARM_DYNXTALK_NODETECT_AMB_THRESHOLD_KCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 132))
#define VL53LX_TUNINGPARM_DYNXTALK_NODETECT_SAMPLE_LIMIT \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 133))
#define VL53LX_TUNINGPARM_DYNXTALK_NODETECT_XTALK_OFFSET_KCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 134))
#define VL53LX_TUNINGPARM_DYNXTALK_NODETECT_MIN_RANGE_MM \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 135))
#define VL53LX_TUNINGPARM_LOWPOWERAUTO_VHV_LOOP_BOUND \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 136))
#define VL53LX_TUNINGPARM_LOWPOWERAUTO_MM_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 137))
#define VL53LX_TUNINGPARM_LOWPOWERAUTO_RANGE_CONFIG_TIMEOUT_US \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 138))
#define VL53LX_TUNINGPARM_VERY_SHORT_DSS_RATE_MCPS \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 139))
#define VL53LX_TUNINGPARM_PHASECAL_PATCH_POWER \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 140))
#define VL53LX_TUNINGPARM_HIST_MERGE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 141))
#define VL53LX_TUNINGPARM_RESET_MERGE_THRESHOLD \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 142))
#define VL53LX_TUNINGPARM_HIST_MERGE_MAX_SIZE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 143))
#define VL53LX_TUNINGPARM_DYNXTALK_MAX_SMUDGE_FACTOR \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 144))
#define VL53LX_TUNINGPARM_UWR_ENABLE \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 145))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_1_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 146))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_1_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 147))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_2_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 148))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_2_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 149))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_3_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 150))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_3_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 151))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_4_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 152))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_4_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 153))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_5_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 154))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_ZONE_5_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 155))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_1_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 156))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_1_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 157))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_2_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 158))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_2_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 159))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_3_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 160))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_3_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 161))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_4_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 162))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_4_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 163))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_5_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 164))
#define VL53LX_TUNINGPARM_UWR_MEDIUM_CORRECTION_ZONE_5_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 165))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_1_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 166))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_1_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 167))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_2_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 168))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_2_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 169))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_3_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 170))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_3_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 171))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_4_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 172))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_4_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 173))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_5_MIN \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 174))
#define VL53LX_TUNINGPARM_UWR_LONG_ZONE_5_MAX \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 175))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_1_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 176))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_1_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 177))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_2_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 178))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_2_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 179))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_3_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 180))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_3_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 181))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_4_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 182))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_4_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 183))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_5_RANGEA \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 184))
#define VL53LX_TUNINGPARM_UWR_LONG_CORRECTION_ZONE_5_RANGEB \
((VL53LX_TuningParms) (VL53LX_TUNINGPARM_PUBLIC_PAGE_BASE_ADDRESS + 185))




#endif





