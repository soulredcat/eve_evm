// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{FRAGMENT, assert_load_rejected, setup};

#[test]
fn fragments_reject_unknown_fields_unsupported_versions_and_non_adapter_grants() {
    for encoded in [
        "version = 2\nadapters = []\n",
        "version = 1\nadapters = []\nignore_all_source = true\n",
        "version = 1\nadapters = []\nexclusions = []\n",
        "version = 1\nadapters = []\nsize_reviews = []\n",
        "version = 1\nadapters = []\nexceptions = []\n",
        "version = 1\nadapters = []\ngenerated_modules = []\n",
        "version = 1\nadapters = []\nadapter_files = []\n",
        "version = 1\ncurrent_bulk = 4\nadapters = []\n",
        "version = 1\n[[adapters]]\npath = \"validator/src/codec_adapter.rs\"\n",
        "version = \"one\"\nadapters = []\n",
        "version = 1\n",
    ] {
        let fixture = setup();
        fixture.write(FRAGMENT, encoded);
        assert_load_rejected(&fixture);
    }
}
