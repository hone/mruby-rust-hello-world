MRuby::Gem::Specification.new('mruby-rust') do |spec|
	spec.license = 'MIT'
	spec.authors = 'Terence Lee'
	spec.version = '0.0.1'
	spec.description = 'Rust from Mruby'
  spec.bins = ["mruby-rust"]

  spec.add_dependency 'mruby-print', core: 'mruby-print'

  require 'open3'
  def run_command env, command
    STDOUT.sync = true
    puts "build: [exec] #{command}"
    Open3.popen2e(env, command) do |stdin, stdout, thread|
      print stdout.read
      fail "#{command} failed" if thread.value != 0
    end
  end

  FileUtils.mkdir_p build_dir

  e = {
    "CARGO_TARGET_DIR" => build_dir
  }
  run_command e, "cargo build"
  spec.linker.libraries << 'foo'
  spec.linker.libraries << 'pthread'
  spec.linker.libraries << 'dl'
  spec.linker.library_paths << "#{build_dir}/debug"
  spec.linker.flags << "-Wl,--undefined=tmrb_nil_value"
end
