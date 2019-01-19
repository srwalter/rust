use crate::utils::paths;
use crate::utils::{is_copy, match_type, snippet, span_lint, span_note_and_lint};
use rustc::hir;
use rustc::lint::LateContext;

use super::OPTION_MAP_UNWRAP_OR;

/// lint use of `map().unwrap_or()` for `Option`s
pub(super) fn lint(cx: &LateContext<'_, '_>, expr: &hir::Expr, map_args: &[hir::Expr], unwrap_args: &[hir::Expr]) {
    // lint if the caller of `map()` is an `Option`
    let unwrap_ty = cx.tables.expr_ty(&unwrap_args[1]);
    if match_type(cx, cx.tables.expr_ty(&map_args[0]), &paths::OPTION) && is_copy(cx, unwrap_ty) {
        // get snippets for args to map() and unwrap_or()
        let map_snippet = snippet(cx, map_args[1].span, "..");
        let unwrap_snippet = snippet(cx, unwrap_args[1].span, "..");
        // lint message
        // comparing the snippet from source to raw text ("None") below is safe
        // because we already have checked the type.
        let arg = if unwrap_snippet == "None" { "None" } else { "a" };
        let suggest = if unwrap_snippet == "None" {
            "and_then(f)"
        } else {
            "map_or(a, f)"
        };
        let msg = &format!(
            "called `map(f).unwrap_or({})` on an Option value. \
             This can be done more directly by calling `{}` instead",
            arg, suggest
        );
        // lint, with note if neither arg is > 1 line and both map() and
        // unwrap_or() have the same span
        let multiline = map_snippet.lines().count() > 1 || unwrap_snippet.lines().count() > 1;
        let same_span = map_args[1].span.ctxt() == unwrap_args[1].span.ctxt();
        if same_span && !multiline {
            let suggest = if unwrap_snippet == "None" {
                format!("and_then({})", map_snippet)
            } else {
                format!("map_or({}, {})", unwrap_snippet, map_snippet)
            };
            let note = format!(
                "replace `map({}).unwrap_or({})` with `{}`",
                map_snippet, unwrap_snippet, suggest
            );
            span_note_and_lint(cx, OPTION_MAP_UNWRAP_OR, expr.span, msg, expr.span, &note);
        } else if same_span && multiline {
            span_lint(cx, OPTION_MAP_UNWRAP_OR, expr.span, msg);
        };
    }
}
