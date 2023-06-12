# typed: strict
# frozen_string_literal: true

require "parallel"

module Packwerk
  class ParseRun
    extend T::Sig

    GetOffensesProc = T.type_alias do
      T.proc.params(processed_file: FileProcessor::ProcessedFile).returns(T::Array[Offense])
    end

    sig do
      params(
        relative_file_set: FilesForProcessing::RelativeFileSet,
        parallel: T::Boolean,
      ).void
    end
    def initialize(relative_file_set:, parallel:)
      @relative_file_set = relative_file_set
      @parallel = parallel
    end

    sig do
      params(
        run_context: RunContext,
        on_interrupt: T.nilable(T.proc.void),
        block: T.nilable(T.proc.params(
          offenses: T::Array[Packwerk::Offense],
        ).void)
      ).returns(T::Array[Offense])
    end
    def find_offenses(run_context, on_interrupt: nil, &block)
      get_offenses_proc = get_offenses_proc(run_context, &block)

      if !@parallel
        raise "Serial mode not supported for now"
      end

      cache_dir = run_context.cache_directory

      processed_files = RustParser.get_unresolved_references(Pathname.pwd, cache_dir, @relative_file_set.to_a)
      Parallel.flat_map(processed_files, &get_offenses_proc)
    end

    private

    sig do
      params(
        run_context: RunContext,
        block: T.nilable(T.proc.params(offenses: T::Array[Offense]).void)
      ).returns(GetOffensesProc)
    end
    def get_offenses_proc(run_context, &block)
      if block
        T.let(proc do |processed_file|
          run_context.offenses_for_processed_file(processed_file: processed_file).tap(&block)
        end, GetOffensesProc)
      else
        T.let(proc do |processed_file|
          run_context.offenses_for_processed_file(processed_file: processed_file)
        end, GetOffensesProc)
      end
    end
  end

  private_constant :ParseRun
end
