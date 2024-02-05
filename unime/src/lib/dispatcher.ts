import type { Action } from '@bindings/actions/Action';
import { sanitizeStringify } from './safe-logging';

import { invoke } from '@tauri-apps/api/core';
import { info } from '@tauri-apps/plugin-log';

/**
 * Dispatches an action to the Tauri backend.
 *
 * @param {Action} action
 */
export const dispatch = async (action: Action) => {
    info(`Dispatching action: ${sanitizeStringify(action)}`);
    await invoke('handle_action', { action }).catch((err) => {
        console.error(err);
    });
};
