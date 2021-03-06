/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use ipc_channel::ipc::IpcSender;
use msg::constellation_msg::PipelineId;
use rust_webvr_api::*;

pub type WebVRResult<T> = Result<T, String>;

// Messages from Script thread to WebVR thread.
#[derive(Debug, Deserialize, Serialize)]
pub enum WebVRMsg {
    RegisterContext(PipelineId),
    UnregisterContext(PipelineId),
    PollEvents(IpcSender<bool>),
    GetDisplays(IpcSender<WebVRResult<Vec<VRDisplayData>>>),
    GetFrameData(
        PipelineId,
        u32,
        f64,
        f64,
        IpcSender<WebVRResult<VRFrameData>>,
    ),
    ResetPose(PipelineId, u32, IpcSender<WebVRResult<VRDisplayData>>),
    RequestPresent(PipelineId, u32, IpcSender<WebVRResult<()>>),
    ExitPresent(PipelineId, u32, Option<IpcSender<WebVRResult<()>>>),
    CreateCompositor(u32),
    CreateMockDisplay,
    MessageMockDisplay(MockVRControlMsg),
    GetGamepads(
        Vec<u32>,
        IpcSender<WebVRResult<Vec<(Option<VRGamepadData>, VRGamepadState)>>>,
    ),
    GetGamepadsForDisplay(
        u32,
        IpcSender<WebVRResult<Vec<(VRGamepadData, VRGamepadState)>>>,
    ),
    Exit,
}
