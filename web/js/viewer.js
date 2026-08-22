// SessionLedger viewer — pure JS event filter and detail rendering.

(function() {
  'use strict';

  function init() {
    var select = document.getElementById('search');
    var rows = document.querySelectorAll('#sessions tbody tr');
    if (!select || rows.length === 0) return;

    select.addEventListener('change', function() {
      var tool = select.value;
      rows.forEach(function(row) {
        var cell = row.cells[1];
        if (!cell) return;
        if (!tool || cell.textContent.trim() === tool) {
          row.style.display = '';
        } else {
          row.style.display = 'none';
        }
      });
    });

    rows.forEach(function(row) {
      row.addEventListener('click', function() {
        showDetail(row);
      });
    });
  }

  function showDetail(row) {
    var section = document.getElementById('detail');
    var content = document.getElementById('detail-content');
    if (!section || !content) return;

    var sessionId = row.cells[0] ? row.cells[0].textContent.trim() : 'unknown';
    var tool = row.cells[1] ? row.cells[1].textContent.trim() : 'unknown';
    var tokens = row.cells[3] ? row.cells[3].textContent.trim() : '0';
    var events = row.cells[4] ? row.cells[4].textContent.trim() : '0';

    section.hidden = false;
    content.textContent = [
      'Session: ' + sessionId,
      'Tool: ' + tool,
      'Tokens: ' + tokens,
      'Events: ' + events,
      '',
      'OKF bundle contents:',
      '  manifest.json   — schema version, hashes, compression info',
      '  events.jsonl    — ' + events + ' normalized events',
      '  sidecars/       — traceparent, attribution, fork-of pointers',
      '  provenance.json — ingestion adapter + worker signature'
    ].join('\n');
  }

  document.addEventListener('DOMContentLoaded', init);
})();