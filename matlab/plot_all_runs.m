function plot_all_runs(in_file)
%in file is matrix, colums are time steps, rows are trials
% like plot_ave, but only does one run
M = dlmread(in_file);
plot(M);
set(gca,'FontSize',18)
end

