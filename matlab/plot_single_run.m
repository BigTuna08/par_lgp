function plot_single_run(in_file, trial, max_gen)
%in file is matrix, colums are time steps, rows are trials
% like plot_ave, but only does one run
M = dlmread(in_file);
max_gen = 25000000;
% x = linspace(0,max_gen,length(M(trial,:) ));
step = max_gen/length(M(trial,:));
final = M(trial,end)
x = 1:step:max_gen;
plot(x,M(trial,:));
p = gca;
p.XAxis.Exponent = 6;
xticks(1:10^6:max_gen);
set(gca,'FontSize',18)
end


