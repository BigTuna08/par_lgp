function plot_diff(root_dir, gens)

if ~exist('gens','var')
   gens = 10^6; 
end

M_test = dlmread( strcat(root_dir, '/test_fits/best.txt') );
M_cv = dlmread( strcat(root_dir, '/cv_fits/best.txt') );
M = M_test - M_cv; %in file is matrix, colums are time steps, rows are trials
N_points = size(M,2);

ave = mean(M);
sd = std(M); %st dev

step = gens/N_points;
x = 1:step:gens;

fin = ave(end);
fin_d = sd(end);
at_end = [fin - fin_d; fin; fin + fin_d]

plot(x, ave-sd, x, ave, x, ave+sd);
xlabel('evaluations')
set(gca,'FontSize',18)
